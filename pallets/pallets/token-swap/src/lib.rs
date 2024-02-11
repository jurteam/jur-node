//! # Jur Token Swap Pallet
//! A pallet allowing users to submit a proof of deposit along with a signed message
//! containing their Substrate address but signed with the Ethereum key (a secp256k1 signature).
//!
//! ## Motivation
//!
//! Currently JUR Token is hosted on VeChain (https://www.vechain.org/) with
//! most Token probably being on the OceanEx exchange. Users need to be able to migrate their
//! tokens over to the Polkadot/Substrate ecosystem. As many tokens are on exchanges taking
//! a snapshot won’t work well.
//! Instead users need to be able to migrate their tokens on their own.
//!
//! VeChain is an EVM-based chain written in Go. It is not a fork of Go-Ethereum
//! but uses EVM and the same state model.
//! There are a few changes which affect the Merkle trees, primarily the hash function and
//! the account structure.
//! It is not bridged to any other chain currently.
//!
//! ## Design
//!
//! To avoid having to either do a centralized token swap or a sophisticated bridge, a simple
//! scheme was devised where tokens are locked in a contract on a smart contract on the VeChain
//! side and the storage of that contract is verified on the Substrate side. In order to verify
//! storage proofs Substrate needs to be aware of a recent VeChain state root.
//! The correct state root can be easily verified by an auditing user on a VeChain block explorer.
//! As long as the stored values in the smart contract only keep increasing
//! (such as the total value locked per account) it is fine if the state root is older.
//! The increased values can be proven once a newer state root is present.
//!
//! To prove a storage value from a state root several steps are needed:
//! From the stateRoot field in the blockhash a MPT-Proof leads to the rlp-encoded
//! account structure. The key in this MPT is the hashed address of the smart contract
//! (which is a constant on the Substrate side).
//! From the storageRoot field in the account structure a MPT-Proof can prove
//! a single storage value. The key is the hash of the storage slot. The storage slot depends
//! on the eth address in question and the exact layout of the smart contract.
//! Most likely it will be something like keccak256(bytes32(address) ++ bytes32(0))
//!
//! The smart contract will record the total amount of JUR token an Ethereum address has locked
//! into the contract. This value increases if the same address locks more tokens later.
//! JUR tokens never leave the contract on the VeChain side.
//!
//! On the Substrate side users can submit a proof of deposit along with a signed message
//! containing their Substrate address but signed with the Ethereum key (a secp256k1 signature).
//! The pallet keeps track of how much JUR was already redeemed for this Ethereum address and mints
//! new tokens to the Substrate address if the value has increased.
//!
//!  ## Overview
//!
//! * Keep track of a VeChain state root hash
//! * Verification of storageRoot
//! * Claim call

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::*,
	traits::{
		fungible::{Inspect, Mutate},
		fungibles::{Inspect as Inspects, Mutate as Mutates},
		Get,
	},
};
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use primitives::proof::{
	compute_storage_key_for_depositor, convert, decode_rlp, extract_storage_root, verify_proof,
};
use primitives::{
	Balance, CurrencyId, EthereumAddress, VechainHash, ACCOUNT_ID_INITIAL_INDEX, ADDRESS_LEN,
	ETHEREUM_SIGNATURE_SIZE, FROM_INDEX, MAX_ACCOUNT_ID_INDEX,
};
use scale_info::TypeInfo;
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::blake2_256, hashing::keccak_256};
use sp_runtime::traits::Zero;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

/// Ethereum Signature
#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct EcdsaSignature(pub [u8; ETHEREUM_SIGNATURE_SIZE]);

impl PartialEq for EcdsaSignature {
	fn eq(&self, other: &Self) -> bool {
		self.0[..] == other.0[..]
	}
}

impl sp_std::fmt::Debug for EcdsaSignature {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "EcdsaSignature({:?})", &self.0[..])
	}
}

/// Information concerning the VeChain state root information
#[derive(Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RootInfo<BlockNumber> {
	/// VeChain state root hash
	pub storage_root: Vec<u8>,
	///  An unverified block number as metadata for users
	pub meta_block_number: BlockNumber,
	/// An unverified string (IPFS path) as metadata for users
	pub ipfs_path: Vec<u8>,
}

/// Asset id type alias.
type AssetIdOf<T> =
	<<T as Config>::Assets as Inspects<<T as frame_system::Config>::AccountId>>::AssetId;

/// Balance type alias.
type BalanceOf<T> =
	<<T as Config>::Assets as Inspects<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::LockableCurrency;
	use frame_system::pallet_prelude::*;
	use primitives::{ValidityError, ETHEREUM_ADDRESS_SIZE, PRIORITY};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// eth address of the deposit contract
		#[pallet::constant]
		type DepositContractAddress: Get<EthereumAddress>;

		/// Jur Specific Prefix
		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		/// Assets for deposit/withdraw  assets to/from token-swap module
		type Assets: Inspects<Self::AccountId, AssetId = CurrencyId, Balance = Balance>
			+ Mutates<Self::AccountId, AssetId = CurrencyId, Balance = Balance>;

		/// Balances for deposit/withdraw  balance to/from account
		type Balances: Inspect<Self::AccountId, Balance = Balance>
			+ Mutate<Self::AccountId, Balance = Balance>
			+ LockableCurrency<Self::AccountId, Balance = Balance, Moment = BlockNumberFor<Self>>;

		/// The asset id for native currency.
		#[pallet::constant]
		type NativeCurrencyId: Get<AssetIdOf<Self>>;

		/// Specify which origin is allowed to update storage root.
		type StorageRootOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store new latest claimed balance
	#[pallet::storage]
	#[pallet::getter(fn latest_claimed_balance)]
	pub type LatestClaimedBalance<T> = StorageMap<_, Identity, EthereumAddress, BalanceOf<T>>;

	/// Store VeChain state root
	#[pallet::storage]
	#[pallet::getter(fn root_information)]
	pub type RootInformation<T: Config> = StorageValue<_, RootInfo<BlockNumberFor<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Stored claimed balance [account_id, balance, ethereum_address]
		ClaimedToken(T::AccountId, BalanceOf<T>, [u8; ETHEREUM_ADDRESS_SIZE]),
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
		/// Invalid Substrate Address
		InvalidSubstrateAddress,
		/// Prefix does not match
		PrefixDoesNotMatch,
		/// Invalid Input
		InvalidInput,
	}

	impl<T> From<Error<T>> for TransactionValidityError {
		fn from(error: Error<T>) -> Self {
			match error {
				Error::<T>::InvalidEthereumSignature => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::InvalidEthereumSignature.into()),
				),
				Error::<T>::InvalidSubstrateAddress => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::InvalidSubstrateAddress.into()),
				),
				Error::<T>::PrefixDoesNotMatch => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::PrefixDoesNotMatch.into()),
				),
				Error::<T>::ContentNotFound => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::ContentNotFound.into()),
				),
				Error::<T>::InvalidJson => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::InvalidJson.into()),
				),
				Error::<T>::NotSufficientLockedBalance => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::NotSufficientLockedBalance.into()),
				),
				Error::<T>::InvalidProof => TransactionValidityError::Invalid(
					InvalidTransaction::Custom(ValidityError::InvalidProof.into()),
				),
				_ => TransactionValidityError::Invalid(InvalidTransaction::Custom(
					ValidityError::InvalidInput.into(),
				)),
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Caller provides
		/// * signed message from the Ethereum address which includes the Substrate account id and some jur-specific prefix
		///		** uses the vechain Certification signing standard
		///	    ** The message is “My Jur Address is SUBSTRATE_ADDRESS”
		/// * proof from storageRoot to their latest locked balance
		///
		/// Functionalities:
		/// - Verify the signed message to extract Ethereum address
		/// - Verify the proof for the latest locked balance
		/// - Verify that locked balance is greater than latest claimed balance
		/// - Mint the difference in native currency
		/// - Store new latest claimed balance
		///
		/// Emits `ClaimedToken` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::claim())]
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

		/// Store VeChain state root information.
		/// It can be updated by the root origin (e.g. through sudo or governance).
		///
		/// Emits `UpdatedStorageRoot` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update_state_root())]
		pub fn update_state_root(
			origin: OriginFor<T>,
			vechain_root_hash: VechainHash,
			meta_block_number: BlockNumberFor<T>,
			ipfs_path: Vec<u8>,
			account_proof: Vec<Vec<u8>>,
		) -> DispatchResult {
			T::StorageRootOrigin::ensure_origin(origin)?;

			let deposit_contract_hash: Vec<u8> =
				blake2_256(&T::DepositContractAddress::get().0).to_vec();
			let account_rlp = verify_proof(vechain_root_hash, account_proof, deposit_contract_hash)
				.ok()
				.ok_or(Error::<T>::InvalidProof)?;
			let storage_root = extract_storage_root(account_rlp)
				.ok()
				.ok_or(Error::<T>::InvalidProof)?;

			let root_information =
				RootInfo { storage_root: storage_root.clone(), meta_block_number, ipfs_path };

			RootInformation::<T>::put(root_information);
			Self::deposit_event(Event::<T>::UpdatedStorageRoot(storage_root));
			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Verification of storageRoot. This proof is verified once at the same time as the update of the state root.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			let (maybe_signer, maybe_json, storage_proof) = match call {
				Call::claim { ethereum_signature, signed_json, storage_proof } => {
					let blake2_256_hash: VechainHash = blake2_256(signed_json);
					(
						Self::eth_recover(ethereum_signature, blake2_256_hash),
						serde_json::from_slice(signed_json),
						storage_proof,
					)
				},
				_ => return Err(InvalidTransaction::Call.into()),
			};

			let signer = maybe_signer.ok_or(InvalidTransaction::Custom(
				ValidityError::InvalidEthereumSignature.into(),
			))?;

			let vs: serde_json::Value = maybe_json
				.ok()
				.ok_or(InvalidTransaction::Custom(ValidityError::InvalidJson.into()))?;
			let s_proof = storage_proof.clone();

			Self::get_address(vs)?;
			Self::get_mint_amount(signer, s_proof)?;

			Ok(ValidTransaction {
				priority: PRIORITY,
				requires: vec![],
				provides: vec![("claims", signer).encode()],
				longevity: TransactionLongevity::max_value(),
				propagate: true,
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	fn get_address(signed_json: serde_json::Value) -> Result<Vec<u8>, Error<T>> {
		// Step-2: Parse signed json as json and extract the payload >> content.
		// Extract Substrate address after removing refix 'My JUR address is' and
		// convert into T::AccountId and remove dest parameter

		let content_str = signed_json["payload"]["content"]
			.as_str()
			.ok_or(Error::<T>::ContentNotFound)?;

		ensure!(
			content_str.as_bytes().starts_with(T::Prefix::get()),
			Error::<T>::PrefixDoesNotMatch
		);

		let substrate_address = &content_str[T::Prefix::get().len()..];
		let address = bs58::decode(substrate_address)
			.into_vec()
			.ok()
			.ok_or(Error::<T>::InvalidSubstrateAddress)?;
		ensure!(address.len() == ADDRESS_LEN, Error::<T>::InvalidSubstrateAddress);

		Ok(address)
	}
	fn get_mint_amount(
		signer: EthereumAddress,
		storage_proof: Vec<Vec<u8>>,
	) -> Result<(Balance, Balance), Error<T>> {
		// Step-3: Proof Verification

		let storage_key = compute_storage_key_for_depositor(signer);
		let storage_rlp = verify_proof(
			convert(Self::root_information().storage_root)
				.ok()
				.ok_or(Error::<T>::InvalidInput)?,
			storage_proof,
			storage_key,
		)
		.ok()
		.ok_or(Error::<T>::InvalidProof)?;

		let locked_balance = decode_rlp(storage_rlp)
			.ok()
			.ok_or(Error::<T>::InvalidProof)?;
		let balance = Self::latest_claimed_balance(signer).unwrap_or(Zero::zero());
		ensure!(locked_balance > balance, Error::<T>::NotSufficientLockedBalance);

		Ok((locked_balance, locked_balance - balance))
	}

	fn process_claim(
		ethereum_signature: EcdsaSignature,
		signed_json: Vec<u8>,
		storage_proof: Vec<Vec<u8>>,
	) -> DispatchResult {
		// Step: 1 Recover signer from signed json
		let blake2_256_hash: VechainHash = blake2_256(&signed_json);

		let signer = Self::eth_recover(&ethereum_signature, blake2_256_hash)
			.ok_or(Error::<T>::InvalidEthereumSignature)?;

		let vs: serde_json::Value = serde_json::from_slice(&signed_json)
			.ok()
			.ok_or(Error::<T>::InvalidJson)?;

		let address = Self::get_address(vs)?;
		let account_id =
			T::AccountId::decode(&mut &address[ACCOUNT_ID_INITIAL_INDEX..MAX_ACCOUNT_ID_INDEX])
				.map_err(|_| Error::<T>::InvalidJson)?;
		let (locked_balance, mint_amount) = Self::get_mint_amount(signer, storage_proof)?;
		T::Balances::mint_into(&account_id, mint_amount)?;

		LatestClaimedBalance::<T>::insert(signer, locked_balance);
		Self::deposit_event(Event::<T>::ClaimedToken(account_id, locked_balance, signer.0));
		Ok(())
	}

	// Attempts to recover the Ethereum address from a message signature signed by using
	// the Ethereum RPC's `personal_sign` and `eth_sign`.
	fn eth_recover(s: &EcdsaSignature, blake2_256_hash: VechainHash) -> Option<EthereumAddress> {
		let mut res = EthereumAddress::default();
		res.0.copy_from_slice(
			&keccak_256(&secp256k1_ecdsa_recover(&s.0, &blake2_256_hash).ok()?[..])[FROM_INDEX..],
		);
		Some(res)
	}
}
