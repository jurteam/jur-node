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
use primitives::{Balance, CurrencyId, EthereumAddress, RootHash};
use rlp::DecoderError;
use scale_info::TypeInfo;
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::blake2_256, hashing::keccak_256};
use sp_runtime::traits::Zero;
use sp_std::prelude::*;

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

		/// The pallet needs to keep track of a Vechain state root hash
		#[pallet::constant]
		type VechainRootHash: Get<RootHash>;

		/// eth address of the deposit contract
		#[pallet::constant]
		type EthAddress: Get<EthereumAddress>;

		/// Have an unverified block number as metadata for users
		#[pallet::constant]
		type MetaBlockNumber: Get<Self::BlockNumber>;

		/// Maximum number of prices.
		#[pallet::constant]
		type IPFSPath: Get<Vec<u8>>;

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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn latest_claimed_balance)]
	pub type LatestClaimedBalance<T> = StorageMap<_, Identity, EthereumAddress, BalanceOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Stored claimed balance [balance]
		ClaimedBalanceStored(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid JSON
		InvalidJson,
		/// Content Not Found
		ContentNotFound,
		/// A needed statement was not included.
		InvalidStatement,
		/// Invalid Ethereum signature.
		InvalidEthereumSignature,
		/// Not Sufficient locked balance.
		NotSufficientLockedBalance,
		/// Invalid proof
		InvalidProof,
		NotImplemented,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn claim(
			origin: OriginFor<T>,
			locked_balance: BalanceOf<T>,
			ethereum_signature: EcdsaSignature,
			signed_json: Vec<u8>,
		) -> DispatchResult {
			ensure_none(origin)?;

			Self::process_claim(locked_balance, ethereum_signature, signed_json)?;
			Ok(())
		}
	}
}

impl<T> From<rlp::DecoderError> for Error<T> {
	fn from(_: DecoderError) -> Self {
		Error::<T>::InvalidProof
	}
}

impl<T: Config> Pallet<T> {
	fn process_claim(
		locked_balance: BalanceOf<T>,
		ethereum_signature: EcdsaSignature,
		signed_json: Vec<u8>,
	) -> DispatchResult {
		// Step: 1 Recover signer from signed json
		let blake2_256_hash: [u8; 32] = blake2_256(&signed_json);

		let signer = Self::eth_recover(&ethereum_signature, blake2_256_hash)
			.ok_or(Error::<T>::InvalidEthereumSignature)?;

		// Step-2: Parse signed json as json and extract the payload >> content. Extract Substrate address after removing refix 'My JUR address is' and convert into T::AccountId and remove dest parameter

		ensure!(Self::json_validation(signed_json.clone()), Error::<T>::InvalidJson);

		let payload = Self::get_nested_json(signed_json, "payload".as_bytes().to_vec());
		let content = Self::get_json_value(payload, "content".as_bytes().to_vec());
		let content = sp_std::str::from_utf8(&content).unwrap_or("");

		ensure!(!content.is_empty(), Error::<T>::ContentNotFound);

		let substrate_address = &content[T::Prefix::get().len()..];
		let address = bs58::decode(substrate_address).into_vec().unwrap();
		let account_id =
			T::AccountId::decode(&mut &address[1..33]).map_err(|_| Error::<T>::InvalidJson)?;

		/// TODO Step-3: Proof Verification
		let balance = Self::latest_claimed_balance(&signer).unwrap_or(Zero::zero());
		ensure!(locked_balance > balance, Error::<T>::NotSufficientLockedBalance);

		let mint_amount = locked_balance - balance;
		T::Balances::mint_into(&account_id, mint_amount)?;

		LatestClaimedBalance::<T>::insert(signer, locked_balance.clone());
		Self::deposit_event(Event::<T>::ClaimedBalanceStored(locked_balance));
		Ok(())
	}

	fn get_nested_json(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
		let mut result = Vec::new();
		let mut k = Vec::new();
		let keyl = key.len();
		let jl = j.len();
		k.push(b'"');
		for xk in 0..keyl {
			k.push(*key.get(xk).unwrap());
		}
		k.push(b'"');
		k.push(b':');
		let kl = k.len();
		for x in 0..jl {
			let mut m = 0;
			if x + kl > jl {
				break;
			}
			for (xx, i) in (x..x + kl).enumerate() {
				if *j.get(i).unwrap() == *k.get(xx).unwrap() {
					m += 1;
				}
			}
			if m == kl {
				let mut os = true;
				for i in x + kl..jl - 1 {
					if *j.get(i).unwrap() == b'{' && os {
						os = false;
					}
					result.push(*j.get(i).unwrap());
					if *j.get(i).unwrap() == b'}' && !os {
						break;
					}
				}
				break;
			}
		}
		result
	}

	fn json_validation(j: Vec<u8>) -> bool {
		// minimum lenght of 2
		if j.len() < 2 {
			return false;
		}
		// checks star/end with {}
		if *j.get(0).unwrap() == b'{' && *j.last().unwrap() != b'}' {
			return false;
		}
		// checks start/end with []
		if *j.get(0).unwrap() == b'[' && *j.last().unwrap() != b']' {
			return false;
		}
		// check that the start is { or [
		if *j.get(0).unwrap() != b'{' && *j.get(0).unwrap() != b'[' {
			return false;
		}
		//checks that end is } or ]
		if *j.last().unwrap() != b'}' && *j.last().unwrap() != b']' {
			return false;
		}
		//checks " opening/closing and : as separator between name and values
		let mut s: bool = true;
		let mut d: bool = true;
		let mut pg: bool = true;
		let mut ps: bool = true;
		let mut bp = b' ';
		for b in j {
			if b == b'[' && s {
				ps = false;
			}
			if b == b']' && s && !ps {
				ps = true;
			}

			if b == b'{' && s {
				pg = false;
			}
			if b == b'}' && s && !pg {
				pg = true;
			}

			if b == b'"' && s && bp != b'\\' {
				s = false;
				bp = b;
				d = false;
				continue;
			}
			if b == b':' && s {
				d = true;
				bp = b;
				continue;
			}
			if b == b'"' && !s && bp != b'\\' {
				s = true;
				bp = b;
				d = true;
				continue;
			}
			bp = b;
		}

		//fields are not closed properly
		if !s {
			return false;
		}
		//fields are not closed properly
		if !d {
			return false;
		}
		//fields are not closed properly
		if !ps {
			return false;
		}
		//fields are not closed properly
		if !pg {
			return false;
		}
		// every ok returns true
		true
	}

	fn get_json_value(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
		let mut result = Vec::new();
		let mut k = Vec::new();
		let keyl = key.len();
		let jl = j.len();
		k.push(b'"');
		for xk in 0..keyl {
			k.push(*key.get(xk).unwrap());
		}
		k.push(b'"');
		k.push(b':');
		let kl = k.len();
		for x in 0..jl {
			let mut m = 0;
			if x + kl > jl {
				break;
			}
			for (xx, i) in (x..x + kl).enumerate() {
				if *j.get(i).unwrap() == *k.get(xx).unwrap() {
					m += 1;
				}
			}
			if m == kl {
				let mut lb = b' ';
				let mut op = true;
				let mut os = true;
				for i in x + kl..jl - 1 {
					if *j.get(i).unwrap() == b'[' && op && os {
						os = false;
					}
					if *j.get(i).unwrap() == b'}' && op && !os {
						os = true;
					}
					if *j.get(i).unwrap() == b':' && op {
						continue;
					}
					if *j.get(i).unwrap() == b'"' && op && lb != b'\\' {
						op = false;
						continue;
					}
					if *j.get(i).unwrap() == b'"' && !op && lb != b'\\' {
						break;
					}
					if *j.get(i).unwrap() == b'}' && op {
						break;
					}
					if *j.get(i).unwrap() == b']' && op {
						break;
					}
					if *j.get(i).unwrap() == b',' && op && os {
						break;
					}
					result.push(*j.get(i).unwrap());
					lb = *j.get(i).unwrap();
				}
				break;
			}
		}
		result
	}

	fn verify_proof(
		root: RootHash,
		proof: Vec<Vec<u8>>,
		key: Vec<u8>,
	) -> Result<Vec<u8>, Error<T>> {
		let mut nibbles = vec![];

		for (i, k) in key.iter().enumerate() {
			nibbles.push(k >> 4);
			nibbles.push(k % 16);
		}

		let nibbles_iter = nibbles.iter();

		for proof_step in proof.iter() {
			let blake2_256_hash: RootHash = blake2_256(proof_step);

			ensure!(blake2_256_hash == root, Error::<T>::InvalidProof);

			let rlp = rlp::Rlp::new(proof_step);
			match rlp.item_count()? {
				2 => {
					let mut node = rlp.iter();
					let prefix: Vec<u8> = match node.next() {
						Some(n) => n.as_val()?,
						None => return Err(Error::<T>::InvalidProof),
					};

					let value: Vec<u8> = match node.next() {
						Some(n) => n.as_val()?,
						None => return Err(Error::<T>::InvalidProof),
					};

					let odd = prefix[0] & 16 != 0;
					let terminal = prefix[0] & 32 != 0;

					let mut prefix_nibbles = vec![];

					for (i, p) in prefix.iter().enumerate() {
						if i != 0 {
							prefix_nibbles.push(p >> 4);
						}

						if i != 0 || odd {
							prefix_nibbles.push(p % 16);
						}
					}

					let prefix_nibbles_len = prefix_nibbles.len();

					let n = nibbles_iter.take(prefix_nibbles_len);

					//assert_eq!(Some(prefix_nibbles), None);
					/// TODO This is to removed
					return Ok(vec![]);
				},
				17 => return Err(Error::<T>::NotImplemented),
				_ => return Err(Error::<T>::InvalidProof),
			}

			return Err(Error::<T>::NotImplemented);
		}

		Err(Error::<T>::InvalidProof)
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
