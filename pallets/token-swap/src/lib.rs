#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::*,
	traits::{
		tokens::{
			fungible::{Inspect, Mutate, Transfer},
			fungibles::{Inspect as Inspects, Mutate as Mutates, Transfer as Transfers},
		},
		Get,
	},
};
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use primitives::{Balance, CurrencyId, EthereumAddress, VechainHash};
use scale_info::TypeInfo;
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::blake2_256, hashing::keccak_256};
use sp_runtime::traits::Zero;
use sp_std::prelude::*;
use primitives::proof::{compute_storage_key_for_depositor, convert, decode_rlp, extract_storage_root, verify_proof};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct EcdsaSignature(pub [u8; 65]);

impl PartialEq for EcdsaSignature {
	fn eq(&self, other: &Self) -> bool {
		&self.0[..] == &other.0[..]
	}
}

impl sp_std::fmt::Debug for EcdsaSignature {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "EcdsaSignature({:?})", &self.0[..])
	}
}

#[derive(Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RootInfo<BlockNumber> {
	pub storage_root: Vec<u8>,
	pub meta_block_number: BlockNumber,
	pub ipfs_path: Vec<u8>
}


type AssetIdOf<T> =
	<<T as Config>::Assets as Inspects<<T as frame_system::Config>::AccountId>>::AssetId;
type BalanceOf<T> =
	<<T as Config>::Assets as Inspects<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::LockableCurrency;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// eth address of the deposit contract
		#[pallet::constant]
		type DepositContractAddress: Get<EthereumAddress>;

		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		type Assets: Transfers<Self::AccountId, AssetId = CurrencyId, Balance = Balance>
			+ Inspects<Self::AccountId, AssetId = CurrencyId, Balance = Balance>
			+ Mutates<Self::AccountId, AssetId = CurrencyId, Balance = Balance>;

		type Balances: Inspect<Self::AccountId, Balance = Balance>
			+ Mutate<Self::AccountId, Balance = Balance>
			+ Transfer<Self::AccountId, Balance = Balance>
			+ LockableCurrency<Self::AccountId, Balance = Balance, Moment = Self::BlockNumber>;

		#[pallet::constant]
		type NativeCurrencyId: Get<AssetIdOf<Self>>;

		/// Specify which origin is allowed to update storage root.
		type StorageRootOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn latest_claimed_balance)]
	pub type LatestClaimedBalance<T> = StorageMap<_, Identity, EthereumAddress, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn root_information)]
	pub type RootInformation<T: Config> = StorageValue<_, RootInfo<T::BlockNumber>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Stored claimed balance [account_id, balance, ethereum_address]
		ClaimedToken(T::AccountId, BalanceOf<T>, [u8; 20]),
		/// Updated Storage Root [storage_root]
		UpdatedStorageRoot(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid JSON
		InvalidJson,
		/// Content Not Found
		ContentNotFound,
		/// Invalid Ethereum signature.
		InvalidEthereumSignature,
		/// Not Sufficient locked balance.
		NotSufficientLockedBalance,
		/// Invalid proof
		InvalidProof,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn claim(
			origin: OriginFor<T>,
			ethereum_signature: EcdsaSignature,
			signed_json: Vec<u8>,
			storage_proof: Vec<Vec<u8>>,
		) -> DispatchResult {
			ensure_none(origin)?;

			Self::process_claim(ethereum_signature, signed_json, storage_proof)?;
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_state_root(
			origin: OriginFor<T>,
			vechain_root_hash: VechainHash,
			meta_block_number: T::BlockNumber,
			ipfs_path: Vec<u8>,
			account_proof: Vec<Vec<u8>>,
		) -> DispatchResult {
			T::StorageRootOrigin::ensure_origin(origin)?;

			let deposit_contract_hash: Vec<u8> = blake2_256(&T::DepositContractAddress::get().0).to_vec();
			let account_rlp = verify_proof(vechain_root_hash, account_proof, deposit_contract_hash).ok().ok_or(Error::<T>::InvalidProof)?;
			let storage_root = extract_storage_root(account_rlp).ok().ok_or(Error::<T>::InvalidProof)?;

			let root_information = RootInfo { storage_root: storage_root.clone(), meta_block_number, ipfs_path};

			RootInformation::<T>::put(root_information);
			Self::deposit_event(Event::<T>::UpdatedStorageRoot(storage_root));
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn process_claim(
		ethereum_signature: EcdsaSignature,
		signed_json: Vec<u8>,
		storage_proof: Vec<Vec<u8>>,
	) -> DispatchResult {

		// Step: 1 Recover signer from signed json
		let blake2_256_hash: [u8; 32] = blake2_256(&signed_json);

		let signer = Self::eth_recover(&ethereum_signature, blake2_256_hash)
			.ok_or(Error::<T>::InvalidEthereumSignature)?;

		// Step-2: Parse signed json as json and extract the payload >> content. Extract Substrate address after removing refix 'My JUR address is' and convert into T::AccountId and remove dest parameter

		let vs: serde_json::Value = serde_json::from_slice(&signed_json).ok().ok_or(Error::<T>::InvalidJson)?;
		let content_str = vs["payload"]["content"].as_str().ok_or(Error::<T>::ContentNotFound)?;
		let substrate_address = &content_str[T::Prefix::get().len()..];

		let address = bs58::decode(substrate_address).into_vec().unwrap();
		let account_id =
			T::AccountId::decode(&mut &address[1..33]).map_err(|_| Error::<T>::InvalidJson)?;

		// Step-3: Proof Verification

		let storage_key = compute_storage_key_for_depositor(signer);
		let storage_rlp = verify_proof(convert(Self::root_information().storage_root), storage_proof, storage_key).ok().ok_or(Error::<T>::InvalidProof)?;

		let locked_balance = decode_rlp(storage_rlp).ok().ok_or(Error::<T>::InvalidProof)?;
		let balance = Self::latest_claimed_balance(&signer).unwrap_or(Zero::zero());
		ensure!(locked_balance > balance, Error::<T>::NotSufficientLockedBalance);

		let mint_amount = locked_balance - balance;
		T::Balances::mint_into(&account_id, mint_amount)?;

		LatestClaimedBalance::<T>::insert(signer, locked_balance.clone());
		Self::deposit_event(Event::<T>::ClaimedToken(account_id, locked_balance, signer.0));
		Ok(())
	}

	// Attempts to recover the Ethereum address from a message signature signed by using
	// the Ethereum RPC's `personal_sign` and `eth_sign`.
	fn eth_recover(s: &EcdsaSignature, blake2_256_hash: [u8; 32]) -> Option<EthereumAddress> {
		let mut res = EthereumAddress::default();
		res.0.copy_from_slice(
			&keccak_256(&secp256k1_ecdsa_recover(&s.0, &blake2_256_hash).ok()?[..])[12..],
		);
		Some(res)
	}
}
