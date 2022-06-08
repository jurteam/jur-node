#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::*,
	traits::{
		tokens::{
			fungible::{Inspect, Mutate, Transfer},
			fungibles::{Inspect as Inspects, Mutate as Mutates, Transfer as Transfers},
		},
		Get
	},
};
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use primitives::{Balance, CurrencyId, EthereumAddress};
use scale_info::TypeInfo;
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::keccak_256};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{self, Deserialize, Serialize};

/// The kind of statement an account needs to make for a claim to be valid.
#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum StatementKind {
	/// Statement required to be made by non-SAFT holders.
	Regular,
	/// Statement required to be made by SAFT holders.
	Saft,
}

impl Default for StatementKind {
	fn default() -> Self {
		StatementKind::Regular
	}
}

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
		type VechainRootHash: Get<Self::Hash>;

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
	pub type LatestClaimedBalance<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// The statement kind that must be signed, if any.
	#[pallet::storage]
	pub(super) type Signing<T> = StorageMap<_, Identity, EthereumAddress, StatementKind>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Stored claimed balance [balance]
		ClaimedBalanceStored(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// A needed statement was not included.
		InvalidStatement,
		/// Invalid Ethereum signature.
		InvalidEthereumSignature,
		/// Not Sufficient locked balance.
		NotSufficientLockedBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn claim(
			origin: OriginFor<T>,
			dest: T::AccountId,
			locked_balance: BalanceOf<T>,
			ethereum_signature: EcdsaSignature,
		) -> DispatchResult {
			ensure_none(origin)?;

			let data = dest.using_encoded(to_ascii_hex);
			let signer = Self::eth_recover(&ethereum_signature, &data, &[][..])
				.ok_or(Error::<T>::InvalidEthereumSignature)?;
			ensure!(Signing::<T>::get(&signer).is_none(), Error::<T>::InvalidStatement);

			let latest_claimed_balance = Self::latest_claimed_balance();
			ensure!(
				locked_balance > latest_claimed_balance,
				Error::<T>::NotSufficientLockedBalance
			);

			let mint_amount = locked_balance - latest_claimed_balance;
			T::Balances::mint_into(&dest, mint_amount)?;

			LatestClaimedBalance::<T>::put(locked_balance.clone());
			Self::deposit_event(Event::<T>::ClaimedBalanceStored(locked_balance));
			Ok(())
		}
	}
}

/// Converts the given binary data into ASCII-encoded hex. It will be twice the length.
fn to_ascii_hex(data: &[u8]) -> Vec<u8> {
	let mut r = Vec::with_capacity(data.len() * 2);
	let mut push_nibble = |n| r.push(if n < 10 { b'0' + n } else { b'a' - 10 + n });
	for &b in data.iter() {
		push_nibble(b / 16);
		push_nibble(b % 16);
	}
	r
}

impl<T: Config> Pallet<T> {
	// Constructs the message that Ethereum RPC's `personal_sign` and `eth_sign` would sign.
	fn ethereum_signable_message(what: &[u8], extra: &[u8]) -> Vec<u8> {
		let prefix = T::Prefix::get();
		let mut l = prefix.len() + what.len() + extra.len();
		let mut rev = Vec::new();
		while l > 0 {
			rev.push(b'0' + (l % 10) as u8);
			l /= 10;
		}
		let mut v = b"\x19Ethereum Signed Message:\n".to_vec();
		v.extend(rev.into_iter().rev());
		v.extend_from_slice(&prefix[..]);
		v.extend_from_slice(what);
		v.extend_from_slice(extra);
		v
	}

	// Attempts to recover the Ethereum address from a message signature signed by using
	// the Ethereum RPC's `personal_sign` and `eth_sign`.
	fn eth_recover(s: &EcdsaSignature, what: &[u8], extra: &[u8]) -> Option<EthereumAddress> {
		let msg = keccak_256(&Self::ethereum_signable_message(what, extra));
		let mut res = EthereumAddress::default();
		res.0
			.copy_from_slice(&keccak_256(&secp256k1_ecdsa_recover(&s.0, &msg).ok()?[..])[12..]);
		Some(res)
	}
}
