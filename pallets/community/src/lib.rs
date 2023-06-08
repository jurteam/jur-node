//! # Jur Community Pallet
//!
//! A pallet allows any $JUR token holder to create a society/community on the Jur.
//!
//! ## Overview
//!
//! Community will be the central building block of our entire ecosystem.
//! It can organize in various forms. We are currently envisioning three different shapes
//! that the Community can take:
//! * a core Community concept which aggregates members based on values and a set of related properties such as religion and language;
//! * a Nation which is a Community that lives together in a specific physical Territory;
//! * a State which is a body that organizes one or more Nations through a Government.
//!
//! ## Interface
//!
//! * `create_community`
//! * `update_community`
//! * `delete_community`
//! * `add_members`
//!

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, BoundedVec};
pub use pallet::*;
use primitives::Incrementable;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
pub use weights::WeightInfo;

use crate::types::*;

pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::EnsureOriginWithArg};
	use frame_system::pallet_prelude::*;

	use super::*;

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<CommunityId> {
		fn community(i: u32) -> CommunityId;
	}
	#[cfg(feature = "runtime-benchmarks")]
	impl<CommunityId: From<u32>> BenchmarkHelper<CommunityId> for () {
		fn community(i: u32) -> CommunityId {
			i.into()
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the community.
		type CommunityId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// Origins to create community
		type CreateOrigin: EnsureOriginWithArg<
			Self::RuntimeOrigin,
			Self::CommunityId,
			Success = Self::AccountId,
		>;

		/// The maximum length of name.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of community Description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type Helper: BenchmarkHelper<Self::CommunityId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store the community with community id
	#[pallet::storage]
	#[pallet::getter(fn communities)]
	pub type Communities<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Community<T::AccountId, T::Hash, T::NameLimit, T::DescriptionLimit>,
	>;

	/// Stores the `CommunityId` that is going to be used for the next
	/// community. This gets incremented whenever a new community is created.
	#[pallet::storage]
	pub type NextCommunityId<T: Config> = StorageValue<_, T::CommunityId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Community [community, founder]
		CreatedCommunity(T::CommunityId, T::AccountId),
		/// Deleted Community [community]
		DeletedCommunity(T::CommunityId),
		/// Updated Community [community]
		UpdatedCommunity(T::CommunityId),
		/// Updated Community [community]
		AddedMembers(T::CommunityId),
		/// Updated Community Metadata [community]
		UpdatedMetadata(T::CommunityId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Community does not exist.
		CommunityNotExist,
		/// No Permission
		NoPermission,
		/// Invalid name given.
		BadName,
		/// Invalid description given.
		BadDescription,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `logo`: This is an image file (also a GIF is valid) that is uploaded on IPFS.
		/// - `name`: Name of the community
		/// - `description`: Information about community
		/// - `members`: as a Founder I should be able to add members to the society by adding
		/// 				their wallet addresses. Tt’s not required to add members immediately
		/// 			at society’s creation
		/// - `metadata`: Other customizable fields like community_type, custom, language, norms etc.
		///
		/// Emits `CreatedCommunity` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_community())]
		pub fn create_community(
			origin: OriginFor<T>,
			logo: Option<Vec<u8>>,
			name: Vec<u8>,
			description: Option<Vec<u8>>,
			members: Option<Vec<T::AccountId>>,
			metadata: Option<CommunityMetaDataFor<T>>,
		) -> DispatchResult {
			let community_id =
				NextCommunityId::<T>::get().unwrap_or(T::CommunityId::initial_value());

			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Self::do_create_community(
				community_id,
				founder,
				logo,
				name,
				description,
				members,
				metadata,
			)
		}

		/// Delete a particular community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be deleted
		///
		/// Emits `DeletedCommunity` event when successful.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::delete_community())]
		pub fn delete_community(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			let community =
				Communities::<T>::get(community_id).ok_or(Error::<T>::CommunityNotExist)?;

			ensure!(founder == community.founder, Error::<T>::NoPermission);

			// TODO Also need to delete associated proposal
			<Communities<T>>::remove(community_id);

			Self::deposit_event(Event::DeletedCommunity(community_id));

			Ok(())
		}

		/// Update a particular community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `logo`: This is an image file (also a GIF is valid) that is uploaded on IPFS.
		/// - `description`: Information about community
		/// - `community_id`: Id of the community to be updated
		///
		/// Emits `UpdatedCommunity` event when successful.
		///
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::update_community())]
		pub fn update_community(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			logo: Option<Vec<u8>>,
			description: Option<Vec<u8>>,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			let bounded_description: BoundedVec<u8, T::DescriptionLimit> =
				if let Some(desc) = description {
					desc
						.try_into()
						.map_err(|_| Error::<T>::BadDescription)?
				} else {
					Default::default()
				};

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;
				ensure!(founder == community.founder, Error::<T>::NoPermission);
				community.logo = logo;
				community.description = bounded_description;
				Self::deposit_event(Event::UpdatedCommunity(community_id));

				Ok(())
			})
		}

		/// Update a particular community metadata from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated.
		/// - `metadata`: Other customizable fields like community_type, custom, language, norms etc.
		///
		/// Emits `UpdatedMetadata` event when successful.
		///
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::update_community())]
		pub fn update_metadata(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			metadata: CommunityMetaDataFor<T>,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;

				ensure!(founder == community.founder, Error::<T>::NoPermission);

				community.metadata = Option::from(metadata);

				Self::deposit_event(Event::UpdatedMetadata(community_id));

				Ok(())
			})
		}

		/// Update members of a particular community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated
		/// - `members`: Members of teh community
		///
		/// Emits `UpdatedCommunity` event when successful.
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn add_members(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			members: Vec<T::AccountId>,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;
				ensure!(founder == community.founder, Error::<T>::NoPermission);

				let mut community_members = community.members.clone();

				for new_members in members.clone() {
					// checking if member is already present in the community.
					if !community_members.contains(&new_members) {
						community_members.push(new_members.clone());
					}
				}
				community.members = community_members;

				Self::deposit_event(Event::AddedMembers(community_id));

				Ok(())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_community(
		community_id: T::CommunityId,
		founder: T::AccountId,
		logo: Option<Vec<u8>>,
		name: Vec<u8>,
		maybe_description: Option<Vec<u8>>,
		maybe_members: Option<Vec<T::AccountId>>,
		metadata: Option<CommunityMetaDataFor<T>>,
	) -> DispatchResult {
		let bounded_name: BoundedVec<u8, T::NameLimit> =
			name.clone().try_into().map_err(|_| Error::<T>::BadName)?;

		let bounded_description: BoundedVec<u8, T::DescriptionLimit> =
			if let Some(desc) = maybe_description {
			desc
				.try_into()
				.map_err(|_| Error::<T>::BadDescription)?
		} else {
			Default::default()
		};

		let members= if let Some(members) = maybe_members {
			members
		} else {
			Vec::new()
		};

		let community = Community {
			founder: founder.clone(),
			logo,
			name: bounded_name,
			description: bounded_description,
			members,
			metadata,
		};

		<Communities<T>>::insert(community_id, community);

		let next_id = community_id.increment();
		NextCommunityId::<T>::set(Some(next_id));

		Self::deposit_event(Event::CreatedCommunity(community_id, founder));

		Ok(())
	}
}
