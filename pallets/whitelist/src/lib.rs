//! # Jur Whitelist pallet
//!
//! A pallet allows any Sudo User to add users who can create the community on the chain.
//! Also add the admin to manage that users.
//!
//!  ## Overview
//!	This pallet will be the main pallet to add the users as founder
//! and add admins to manage that founders.
//! Currently the sudo user can add/revoke the admin to manage the founders.
//! Sudo and admins can add the users into the founders list.
//!
//! ## Interface
//!
//! * `add_user`
//! * `revoke_user`
//! * `add_admin`
//! * `revoke_admin`
//!

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
// pub use weights::WeightInfo;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
// pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::dispatch::Vec;
	use frame_system::pallet_prelude::*;

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store the users with Account id.
	#[pallet::storage]
	#[pallet::getter(fn founders)]
	pub type Founders<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	/// Admins who manage the founders access.
	#[pallet::storage]
	#[pallet::getter(fn admins)]
	pub type Admins<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added user as founder to create community [user]
		AddedFounder(T::AccountId),
		/// Revoke founder [user]
		RevokedFounder(T::AccountId),
		/// Added admin to manage founders [user]
		AddedAdmin(T::AccountId),
		/// Revoke admin [user]
		RevokedAdmin(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already in the founder.
		AlreadyFounder,
		/// Not a founder user.
		FounderNotExist,
		/// Already admin.
		AlreadyAdmin,
		/// Not a admin.
		AdminNotExist,
		/// No Permission.
		NoPermission,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add the user in founders list.
		///
		/// Parameters:
		/// - `account`: Account Id of user.
		///
		/// Emits `AddedFounder` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(10_000_000)]
		pub fn add_founder(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure!(ensure_root(origin.clone()).is_ok() || Admins::<T>::get().binary_search(&ensure_signed(origin.clone())?).is_ok(), Error::<T>::NoPermission);

			let mut founders = Founders::<T>::get();
			let location = founders.binary_search(&account).err().ok_or(Error::<T>::AlreadyFounder)?;

			// Inserting the data into the storage.
			founders.insert(location, account.clone());

			Founders::<T>::put(&founders);

			Self::deposit_event(Event::AddedFounder(account));
			Ok(())
		}

		/// Remove the user from founders list.
		///
		/// Parameters:
		/// - `account`: Account Id of user.
		///
		/// Emits `RevokedFounder` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(10_000_000)]
		pub fn revoke_founder(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure!(ensure_root(origin.clone()).is_ok() || Admins::<T>::get().binary_search(&ensure_signed(origin.clone())?).is_ok(), Error::<T>::NoPermission);

			let mut founders = Founders::<T>::get();
			let location = founders.binary_search(&account).ok().ok_or(Error::<T>::FounderNotExist)?;

			// Removing the data into the storage.
			founders.remove(location);

			Founders::<T>::put(&founders);

			Self::deposit_event(Event::RevokedFounder(account));
			Ok(())
		}

		/// Add the admin to admin list.
		///
		/// Parameters:
		/// - `account`: Account Id of user.
		///
		/// Emits `AddedAdmin` event when successful.
		///
		#[pallet::call_index(2)]
		#[pallet::weight(10_000_000)]
		pub fn add_admin(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure!(ensure_root(origin.clone()).is_ok(), Error::<T>::NoPermission);

			let mut admins = Admins::<T>::get();
			let location = admins.binary_search(&account).err().ok_or(Error::<T>::AlreadyAdmin)?;

			// Inserting the data into the storage.
			admins.insert(location, account.clone());

			Admins::<T>::put(&admins);

			Self::deposit_event(Event::AddedAdmin(account));
			Ok(())
		}

		/// Remove the admin from admin list.
		///
		/// Parameters:
		/// - `account`: Account Id of user.
		///
		/// Emits `RevokedAdmin` event when successful.
		///
		#[pallet::call_index(3)]
		#[pallet::weight(10_000_000)]
		pub fn revoke_admin(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure!(ensure_root(origin.clone()).is_ok(), Error::<T>::NoPermission);

			let mut admins = Admins::<T>::get();
			let location = admins.binary_search(&account).ok().ok_or(Error::<T>::AdminNotExist)?;

			// Removing the data into the storage.
			admins.remove(location);

			Admins::<T>::put(&admins);

			Self::deposit_event(Event::RevokedAdmin(account));
			Ok(())
		}
	}
}
