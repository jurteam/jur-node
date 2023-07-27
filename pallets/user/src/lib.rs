//! # Jur User Pallet
//!
//! A pallet allows any $JUR token holder to add username and profile image on the Jur.
//!
//!  ## Overview
//!	User pallet will be the core pallet to store the basic details of JUR users.
//! Currently this pallet will store the username and profile image of the JUR user.
//! A user can add/update there username and profile image by using this pallet.
//!
//! ## Interface
//!
//! * `update_user`
//!

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use sp_runtime::RuntimeDebug;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::types::User;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The maximum length of username.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of address(IPFS).
		#[pallet::constant]
		type AddressLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store the users with Account id.
	#[pallet::storage]
	#[pallet::getter(fn users)]
	pub type Users<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T::NameLimit, T::AddressLimit>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Updated user-details by user [user]
		UserDetailsUpdated(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not available.
		UserNotAvailable,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add/Update the user details by any JUR user.
		///
		/// Parameters:
		/// - `username`: Username of the Account.
		/// - `avatar`: This is an image file address(also a GIF is valid) that is uploaded on IPFS.
		///
		/// Emits `UserDetailsUpdated` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(10000)]
		pub fn update_user(
			origin: OriginFor<T>,
			username: Option<BoundedVec<u8, T::NameLimit>>,
			avatar: Option<BoundedVec<u8, T::AddressLimit>>,
		) -> DispatchResult {
			let user = ensure_signed(origin.clone())?;

			// creating the user data structure as per given inputs
			let new_user = User { username, avatar };

			// Inserting the data into the storage.
			Users::<T>::insert(user.clone(), new_user);

			Self::deposit_event(Event::UserDetailsUpdated(user));
			Ok(())
		}
	}
}
