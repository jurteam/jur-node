// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! A pallet for managing validators on Rococo.

use frame_support::ensure;
use pallet_session::{SessionHandler, SessionManager};
use sp_runtime::traits::OpaqueKeys;
use sp_runtime::{DispatchError, KeyTypeId};
use sp_staking::SessionIndex;
use sp_std::vec::Vec;

pub use pallet::*;

type Session<T> = pallet_session::Pallet<T>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::EnsureOrigin};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configuration for the parachain proposer.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config {
		/// The overreaching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Privileged origin that can add or remove validators.
		type PrivilegedOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New validators were added to the set.
		ValidatorsRegistered(Vec<T::ValidatorId>),
		/// Validators were removed from the set.
		ValidatorsDeregistered(Vec<T::ValidatorId>),
		// Validators Registered as in Genesis
		RegisteredGenesisValidators(Vec<(T::AccountId, T::ValidatorId, T::Keys)>),
	}

	/// Validators that should be retired, because their Parachain was deregistered.
	#[pallet::storage]
	pub(crate) type ValidatorsToRetire<T: Config> =
		StorageValue<_, Vec<T::ValidatorId>, ValueQuery>;

	/// Validators that should be added.
	#[pallet::storage]
	pub(crate) type ValidatorsToAdd<T: Config> = StorageValue<_, Vec<T::ValidatorId>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add new validators to the set.
		///
		/// The new validators will be active from current session + 2.
		#[pallet::call_index(0)]
		#[pallet::weight({100_000})]
		pub fn register_validators(
			origin: OriginFor<T>,
			validators: Vec<T::ValidatorId>,
		) -> DispatchResult {
			T::PrivilegedOrigin::ensure_origin(origin)?;

			validators
				.clone()
				.into_iter()
				.for_each(|v| ValidatorsToAdd::<T>::append(v));

			Self::deposit_event(Event::ValidatorsRegistered(validators));
			Ok(())
		}

		/// Remove validators from the set.
		///
		/// The removed validators will be deactivated from current session + 2.
		#[pallet::call_index(1)]
		#[pallet::weight({100_000})]
		pub fn deregister_validators(
			origin: OriginFor<T>,
			validators: Vec<T::ValidatorId>,
		) -> DispatchResult {
			T::PrivilegedOrigin::ensure_origin(origin)?;

			validators
				.clone()
				.into_iter()
				.for_each(|v| ValidatorsToRetire::<T>::append(v));

			Self::deposit_event(Event::ValidatorsDeregistered(validators));
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight({100_000})]
		pub fn session_config(
			origin: OriginFor<T>,
			keys: Vec<(T::AccountId, T::ValidatorId, T::Keys)>,
		) -> DispatchResult {
			T::PrivilegedOrigin::ensure_origin(origin)?;

			if T::SessionHandler::KEY_TYPE_IDS.len() != T::Keys::key_ids().len() {
				panic!("Number of keys in session handler and session keys does not match");
			}

			T::SessionHandler::KEY_TYPE_IDS
				.iter()
				.zip(T::Keys::key_ids())
				.enumerate()
				.for_each(|(i, (sk, kk))| {
					if sk != kk {
						panic!(
							"Session handler and session key expect different key type at index: {}",
							i,
						);
					}
				});

			for (account, val, keys) in keys.iter().cloned() {
				<Pallet<T>>::inner_set_keys(&val, keys)
					.expect("genesis config must not contain duplicates; qed");
				if frame_system::Pallet::<T>::inc_consumers_without_limit(&account).is_err() {
					// This will leak a provider reference, however it only happens once (at
					// genesis) so it's really not a big deal and we assume that the user wants to
					// do this since it's the only way a non-endowed account can contain a session
					// key.
					frame_system::Pallet::<T>::inc_providers(&account);
				}
			}

			let initial_validators: Vec<T::ValidatorId> =
				keys.iter().map(|x| x.1.clone()).collect();

			assert!(
				!initial_validators.is_empty(),
				"Empty validator set for session 0 in genesis block!"
			);

			let queued_keys: Vec<_> = initial_validators
				.iter()
				.cloned()
				.map(|v| {
					(
						v.clone(),
						Pallet::<T>::load_keys(&v).expect("Validator in session 1 missing keys!"),
					)
				})
				.collect();

			pallet_session::Validators::<T>::put(initial_validators);
			pallet_session::QueuedKeys::<T>::put(queued_keys);

			T::SessionManager::start_session(0);

			Self::deposit_event(Event::RegisteredGenesisValidators(keys));
			Ok(())
		}
	}
}

impl<T: Config> pallet_session::SessionManager<T::ValidatorId> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
		if new_index <= 1 {
			return None;
		}

		let mut validators = Session::<T>::validators();

		ValidatorsToRetire::<T>::take().iter().for_each(|v| {
			if let Some(pos) = validators.iter().position(|r| r == v) {
				validators.swap_remove(pos);
			}
		});

		ValidatorsToAdd::<T>::take().into_iter().for_each(|v| {
			if !validators.contains(&v) {
				validators.push(v);
			}
		});

		Some(validators)
	}

	fn end_session(_: SessionIndex) {}

	fn start_session(_start_index: SessionIndex) {}
}

impl<T: Config> pallet_session::historical::SessionManager<T::ValidatorId, ()> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<(T::ValidatorId, ())>> {
		<Self as pallet_session::SessionManager<_>>::new_session(new_index)
			.map(|r| r.into_iter().map(|v| (v, Default::default())).collect())
	}

	fn start_session(start_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::start_session(start_index)
	}

	fn end_session(end_index: SessionIndex) {
		<Self as pallet_session::SessionManager<_>>::end_session(end_index)
	}
}

impl<T: Config> Pallet<T> {
	fn load_keys(v: &T::ValidatorId) -> Option<T::Keys> {
		pallet_session::NextKeys::<T>::get(v)
	}

	/// Query the owner of a session key by returning the owner's validator ID.
	pub fn key_owner(id: KeyTypeId, key_data: &[u8]) -> Option<T::ValidatorId> {
		pallet_session::KeyOwner::<T>::get((id, key_data))
	}

	fn put_key_owner(id: KeyTypeId, key_data: &[u8], v: &T::ValidatorId) {
		pallet_session::KeyOwner::<T>::insert((id, key_data), v)
	}

	fn clear_key_owner(id: KeyTypeId, key_data: &[u8]) {
		pallet_session::KeyOwner::<T>::remove((id, key_data));
	}

	fn put_keys(v: &T::ValidatorId, keys: &T::Keys) {
		pallet_session::NextKeys::<T>::insert(v, keys);
	}

	/// Perform the set_key operation, checking for duplicates. Does not set `Changed`.
	///
	/// The old keys for this validator are returned, or `None` if there were none.
	///
	/// This does not ensure that the reference counter in system is incremented appropriately, it
	/// must be done by the caller or the keys will be leaked in storage.
	fn inner_set_keys(
		who: &T::ValidatorId,
		keys: T::Keys,
	) -> Result<Option<T::Keys>, DispatchError> {
		let old_keys = Self::load_keys(who);

		for id in T::Keys::key_ids() {
			let key = keys.get_raw(*id);

			// ensure keys are without duplication.
			ensure!(
				Self::key_owner(*id, key).map_or(true, |owner| &owner == who),
				pallet_session::Error::<T>::DuplicatedKey,
			);
		}

		for id in T::Keys::key_ids() {
			let key = keys.get_raw(*id);

			if let Some(old) = old_keys.as_ref().map(|k| k.get_raw(*id)) {
				if key == old {
					continue;
				}

				Self::clear_key_owner(*id, old);
			}

			Self::put_key_owner(*id, key, who);
		}

		Self::put_keys(who, &keys);
		Ok(old_keys)
	}
}
