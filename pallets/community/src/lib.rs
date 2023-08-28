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
//! * `update_metadata`
//! * `delete_community`
//! * `accept_members`
//! * `join_community`
//!

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, traits::Randomness, BoundedVec};
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
pub mod migration;
pub mod weights;

const LOG_TARGET: &str = "runtime::community";

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::EnsureOriginWithArg};
	use frame_system::pallet_prelude::*;

	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(7);

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
	pub trait Config: frame_system::Config + pallet_whitelist::Config {
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

		type MyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

		/// The maximum length of tag.
		#[pallet::constant]
		type TagLimit: Get<u32>;

		/// The maximum length of color.
		#[pallet::constant]
		type ColorLimit: Get<u32>;

		/// The number of community, allowed to create by a founder.
		#[pallet::constant]
		type CommunityLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// To be used in generating refernce number
	#[pallet::storage]
	pub(crate) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Store the community with community id
	#[pallet::storage]
	#[pallet::getter(fn communities)]
	pub type Communities<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Community<T::AccountId, T::NameLimit, T::DescriptionLimit, T::TagLimit, T::ColorLimit>,
	>;

	/// The communities owned by a given account
	#[pallet::storage]
	#[pallet::getter(fn community_account)]
	pub type CommunityAccount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::CommunityId, T::CommunityLimit>,
		ValueQuery
	>;

	/// Stores the `CommunityId` that is going to be used for the next
	/// community. This gets incremented whenever a new community is created.
	#[pallet::storage]
	pub type NextCommunityId<T: Config> = StorageValue<_, T::CommunityId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Community [communityId, referenceId, founder]
		CreatedCommunity(T::CommunityId, [u8; 16], T::AccountId),
		/// Updated Community [community]
		UpdatedCommunity(T::CommunityId),
		/// Updated Community [community]
		AddedMembers(T::CommunityId),
		/// Updated Community Metadata [community]
		UpdatedMetadata(T::CommunityId),
		/// Joined Community [community]
		JoinedCommunity(T::CommunityId),
		/// Leaved Community [community]
		LeavedCommunity(T::CommunityId),
		/// Removed member from community [member]
		RemovedMember(T::AccountId),
		/// Updated Tag And Colors [community]
		UpdatedTagAndColors(T::CommunityId),
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
		/// Already a member of the community.
		AlreadyMember,
		/// Not member of given community.
		NotMember,
		/// Not Allowed For Public Community
		NotAllowedForPublicCommunity,
		/// Invalid tag given.
		BadTag,
		/// Invalid description given.
		BadColor,
		/// Founder not whitelisted.
		FounderNotExist,
		/// Too Many Communities
		TooManyCommunities
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
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_community())]
		pub fn create_community(
			origin: OriginFor<T>,
			logo: Option<Vec<u8>>,
			name: Vec<u8>,
			description: Option<Vec<u8>>,
			members: Option<Vec<T::AccountId>>,
			metadata: Option<CommunityMetaDataFor<T>>,
			category: Category,
			tagline: Option<Vec<u8>>,
			primary_color: Option<Vec<u8>>,
			secondary_color: Option<Vec<u8>>,
		) -> DispatchResult {
			let community_id =
				NextCommunityId::<T>::get().unwrap_or(T::CommunityId::initial_value());

			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			pallet_whitelist::Founders::<T>::get().binary_search(&founder).ok().ok_or(Error::<T>::FounderNotExist)?;

			Self::do_create_community(
				community_id,
				founder,
				logo,
				name,
				description,
				members,
				metadata,
				category,
				tagline,
				primary_color,
				secondary_color
			)
		}

		/// Update a particular community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated.
		/// - `logo`: This is an image file (also a GIF is valid) that is uploaded on IPFS.
		/// - `description`: Information about community.
		///
		/// Emits `UpdatedCommunity` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::update_community())]
		pub fn update_community(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			logo: Option<Vec<u8>>,
			description: Option<Vec<u8>>,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			let bounded_description: BoundedVec<u8, T::DescriptionLimit> =
				if let Some(desc) = description {
					desc.try_into().map_err(|_| Error::<T>::BadDescription)?
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
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::update_metadata())]
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
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::accept_members())]
		pub fn accept_members(
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

		/// Join any particular public community.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated
		///
		/// Emits `JoinedCommunity` event when successful.
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::join_community())]
		pub fn join_community(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
		) -> DispatchResult {
			let member = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;

				let mut community_members = community.members.clone();

				ensure!(community.founder != member, Error::<T>::AlreadyMember);

				ensure!(!community_members.contains(&member), Error::<T>::AlreadyMember);

				community_members.push(member.clone());

				community.members = community_members;

				Self::deposit_event(Event::JoinedCommunity(community_id));

				Ok(())
			})
		}

		/// Leave any particular private/public community.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated
		///
		/// Emits `LeavedCommunity` event when successful.
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::leave_community())]
		pub fn leave_community(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
		) -> DispatchResult {
			let member = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;

				let mut community_members = community.members.clone();

				ensure!(community_members.contains(&member), Error::<T>::NotMember);

				let index = community_members
					.iter()
					.position(|value| *value == member.clone())
					.expect("Member not found.");

				community_members.remove(index);

				community.members = community_members;

				Self::deposit_event(Event::LeavedCommunity(community_id));

				Ok(())
			})
		}

		/// Remove member from private community.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `member`: member Account which founder want to remove from community
		/// - `community_id`: Id of the community to be updated
		///
		/// Emits `RemovedMember` event when successful.
		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::remove_member())]
		pub fn remove_member(
			origin: OriginFor<T>,
			member: T::AccountId,
			community_id: T::CommunityId,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;

				// TODO update below check to restrict this extrinsic for private communities
				// ensure!(community.type == "Private", Error::<T>::NoPermission);

				ensure!(founder == community.founder, Error::<T>::NoPermission);

				let mut community_members = community.members.clone();

				ensure!(community_members.contains(&member), Error::<T>::NotMember);

				let index = community_members
					.iter()
					.position(|value| *value == member.clone())
					.expect("Member not found.");

				community_members.remove(index);

				community.members = community_members;

				Self::deposit_event(Event::RemovedMember(member));

				Ok(())
			})
		}

		/// Update tag and colors of a particular community from a privileged origin.
		///
		/// The origin must conform to `CreateOrigin`.
		///
		/// Parameters:
		/// - `community_id`: Id of the community to be updated.
		/// - `tag`: This is tagline.
		/// - `primary_color`: Primary color, which will be used by the passport generator.
		/// - `secondary_color`: Secondary color, which will be used by the passport generator..
		/// Emits `UpdatedTagAndColors` event when successful.
		///
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::update_passport_metadata())]
		pub fn update_passport_metadata(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			tagline: Option<Vec<u8>>,
			primary_color: Option<Vec<u8>>,
			secondary_color: Option<Vec<u8>>,
		) -> DispatchResult {
			let founder = T::CreateOrigin::ensure_origin(origin, &community_id)?;

			let bounded_tag: BoundedVec<u8, T::TagLimit> =
				if let Some(t) = tagline {
					t.try_into().map_err(|_| Error::<T>::BadTag)?
				} else {
					Default::default()
				};

			let bounded_primary_color: BoundedVec<u8, T::ColorLimit> =
				if let Some(color) = primary_color {
					color.try_into().map_err(|_| Error::<T>::BadColor)?
				} else {
					Default::default()
				};

			let bounded_secondary_color: BoundedVec<u8, T::ColorLimit> =
				if let Some(color) = secondary_color {
					color.try_into().map_err(|_| Error::<T>::BadColor)?
				} else {
					Default::default()
				};

			Communities::<T>::try_mutate(community_id, |maybe_community| {
				let community = maybe_community
					.as_mut()
					.ok_or(Error::<T>::CommunityNotExist)?;

				ensure!(founder == community.founder, Error::<T>::NoPermission);

				community.tag = bounded_tag;
				community.primary_color = bounded_primary_color;
				community.secondary_color = bounded_secondary_color;

				Self::deposit_event(Event::UpdatedTagAndColors(community_id));

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
		category: Category,
		maybe_tag: Option<Vec<u8>>,
		maybe_primary_color: Option<Vec<u8>>,
		maybe_secondary_color: Option<Vec<u8>>,
	) -> DispatchResult {
		let bounded_name: BoundedVec<u8, T::NameLimit> =
			name.clone().try_into().map_err(|_| Error::<T>::BadName)?;

		let bounded_description: BoundedVec<u8, T::DescriptionLimit> =
			if let Some(desc) = maybe_description {
				desc.try_into().map_err(|_| Error::<T>::BadDescription)?
			} else {
				Default::default()
			};

		let bounded_tag: BoundedVec<u8, T::TagLimit> =
			if let Some(tag) = maybe_tag {
				tag.try_into().map_err(|_| Error::<T>::BadTag)?
			} else {
				Default::default()
			};

		let bounded_primary_color: BoundedVec<u8, T::ColorLimit> =
			if let Some(color) = maybe_primary_color {
				color.try_into().map_err(|_| Error::<T>::BadColor)?
			} else {
				Default::default()
			};

		let bounded_secondary_color: BoundedVec<u8, T::ColorLimit> =
			if let Some(color) = maybe_secondary_color {
				color.try_into().map_err(|_| Error::<T>::BadColor)?
			} else {
				Default::default()
			};

		let members = if let Some(members) = maybe_members { members } else { Vec::new() };

		// Random value.
		let nonce = Self::get_and_increment_nonce();
		let random_seed = T::MyRandomness::random(&nonce).encode();

		let random_number = u128::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u32; qed");

		let random_value: [u8; 16] = random_number.to_be_bytes();

		let community = Community {
			founder: founder.clone(),
			logo,
			name: bounded_name,
			description: bounded_description,
			members,
			metadata,
			reference_id: random_value,
			category,
			tag: bounded_tag,
			primary_color: bounded_primary_color,
			secondary_color: bounded_secondary_color
		};

		<CommunityAccount<T>>::try_mutate(founder.clone(), |communities| -> DispatchResult {
			communities
				.try_push(community_id)
				.map_err(|_| Error::<T>::TooManyCommunities)?;
			Ok(())
		})?;
		<Communities<T>>::insert(community_id, community);

		let next_id = community_id.increment();
		NextCommunityId::<T>::set(Some(next_id));

		Self::deposit_event(Event::CreatedCommunity(community_id, random_value, founder));

		Ok(())
	}

	fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
