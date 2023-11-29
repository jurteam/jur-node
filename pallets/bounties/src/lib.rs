//! # Jur Bounties Pallet
//!
//! A pallet allow founders to create the bounties for the community members.
//!
//! ## Overview
//!
//! A Bounty Spending is a reward for a specified body of work or specified set of objectives.
//!
//! ## Interface
//!
//! * `create_bounty`
//! * `update_bounty`
//! * `add_contributor`
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{Bounty, BountyStatus};
use frame_support::{dispatch::DispatchResultWithPostInfo, BoundedVec};
use primitives::{Incrementable, BLOCKS_PER_DAY, BOUNTY_DURATION_LIMIT};
use sp_std::vec::Vec;
use pallet_passport::Passports;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
// pub mod weights;
// pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_community::Config + pallet_passport::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the Bounty.
		type BountyId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The maximum length of Bounty name/title.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of Bounty description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		/// The maximum length of category.
		#[pallet::constant]
		type CategoryLimit: Get<u32>;

		/// The maximum length of address.
		#[pallet::constant]
		type AccountLimit: Get<u32>;

		// Weight information
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store new Bounty with a unique bounty id for a particular community
	#[pallet::storage]
	#[pallet::getter(fn bounties)]
	pub type Bounties<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		T::BountyId,
		Bounty<
			<T as Config>::NameLimit,
			<T as Config>::CategoryLimit,
			T::BadgeNameLimit,
			<T as Config>::DescriptionLimit,
			T::AccountId,
			T::AccountLimit,
		>,
		OptionQuery,
	>;

	/// Stores the `BountyId` that is going to be used for the next bounty.
	/// This gets incremented whenever a new bounty is created.
	#[pallet::storage]
	pub type NextBountyId<T: Config> =
	StorageMap<_, Twox64Concat, T::CommunityId, T::BountyId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Bounty [Bounty Id]
		CreatedBounty(T::BountyId),
		/// Updated Bounty [Bounty Id]
		UpdatedBounty(T::BountyId),
		/// Added bounty contributor to the chain [Bounty Id]
		AddedContributor(T::BountyId),
		/// Completed bounty and badges assigned to the contributors [Bounty Id]
		CompletedBounty(T::BountyId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Community Does Not Exist.
		CommunityDoesNotExist,
		/// NotAllowed
		NotAllowed,
		/// Invalid description given.
		BadDescription,
		/// New account can't be added due to account limit.
		AccountLimitReached,
		/// Invalid bounty duration.
		InvalidBountyDuration,
		/// Passport not available given user.
		PassportNotAvailable,
		/// Bounty not available on the chain.
		BountyNotAvailable,
		/// contributor not participated in bounty.
		ParticipantNotAvailable,
		/// Bounty badge not exist.
		BadgeNotExist,
		/// Bounty is closed as its deadline is exceed.
		BountyClosed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {

			Weight::zero()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new bounty for a particular community from a origin.
		///
		/// The origin must be Signed and the community founder.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `name`: name/title of the bounty.
		/// - `description`: description of the bounty.
		/// - `duration`: Voting duration of the bounty.
		/// 			In case it is flagged as such, the bounty becomes part of the History.
		///
		/// Emits `CreatedBounty` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(1000000)]
		pub fn create_bounty(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
			category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit>,
			badge: BoundedVec<u8, T::BadgeNameLimit>,
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			duration: u32,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			// Ensuring the bounty creator should be founder.
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			// Ensuring the bounty badge should exist in the community badge directory.
			let is_badge_exist = pallet_passport::Badges::<T>::get(community_id, &badge);
			ensure!(is_badge_exist.is_some(), Error::<T>::BadgeNotExist);

			// Ensuring the bounty duration should lies between the given limits.
			ensure!(
				(1..=BOUNTY_DURATION_LIMIT).contains(&duration),
				Error::<T>::InvalidBountyDuration
			);

			Self::do_create_bounty(
				origin,
				community_id,
				name,
				category,
				badge,
				description,
				duration,
			)
		}

		#[pallet::call_index(1)]
		#[pallet::weight(1000000)]
		pub fn update_bounty(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			bounty_id: T::BountyId,
			participants: BoundedVec<T::AccountId, <T as pallet::Config>::AccountLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			// Ensuring the origin should be founder.
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			// Ensuring the members should have the passport.
			ensure!(
				!participants
					.iter()
					.any(|participant| <Passports<T>>::get(community_id, participant).is_none()),
				Error::<T>::PassportNotAvailable
			);

			Bounties::<T>::try_mutate(community_id, &bounty_id, |bounty_details| {
				let bounty = bounty_details
					.as_mut()
					.ok_or(Error::<T>::BountyNotAvailable)?;

				ensure!((bounty.status == BountyStatus::Ongoing) || (bounty.status == BountyStatus::WorkInProgress), Error::<T>::BountyClosed);

				bounty.participants = participants;
				bounty.status = BountyStatus::WorkInProgress;

				Self::deposit_event(Event::UpdatedBounty(bounty_id));
				Ok(())
			})
		}


		#[pallet::call_index(2)]
		#[pallet::weight(1000000)]
		pub fn complete_bounty(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			bounty_id: T::BountyId,
			contributors: Vec<T::AccountId>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			// Ensuring the members should have the passport.
			ensure!(
				!contributors
					.iter()
					.any(|contributor| <Passports<T>>::get(community_id, contributor).is_none()),
				Error::<T>::PassportNotAvailable
			);


			let bounty_details = <Bounties<T>>::get(community_id, bounty_id).ok_or(Error::<T>::BountyNotAvailable)?;
			let bounty_reward = bounty_details.badge;

			// Issuing the badge to the members
			for member in &contributors {
				Passports::<T>::try_mutate(
					community_id,
					member,
					|passport_details| -> DispatchResult {
						let passport = passport_details
							.as_mut()
							.ok_or(Error::<T>::PassportNotAvailable)?;

						let mut badges = passport.badges.clone();

						if !badges.contains(&bounty_reward) {
							badges.push(bounty_reward.clone());
							passport.badges = badges;
						}

						Ok(())
					},
				)?;
			}


			Bounties::<T>::try_mutate(community_id, &bounty_id, |bounty_details| {
				let bounty = bounty_details
					.as_mut()
					.ok_or(Error::<T>::BountyNotAvailable)?;

				ensure!(bounty.status == BountyStatus::WorkInProgress, Error::<T>::BountyClosed);

				// Ensuring that the contributor should participated in the bounty.
				ensure!(
					contributors
					.iter().
					any(|contributor| bounty.participants.contains(contributor)),
				Error::<T>::ParticipantNotAvailable
				);

				// Adding the new contributors in bounty contributors.
				let mut bounty_contributors = bounty.contributors.clone();

				for new_contributor in contributors.clone() {
					// checking if contributor is already present as bounty contributor.
					if !bounty_contributors.contains(&new_contributor) {
						bounty_contributors.push(new_contributor.clone());
					}
				}
				bounty.contributors = bounty_contributors;


				Self::deposit_event(Event::UpdatedBounty(bounty_id));
				Ok(())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_bounty(
		creator: T::AccountId,
		community_id: T::CommunityId,
		name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
		category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit>,
		badge: BoundedVec<u8, T::BadgeNameLimit>,
		description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
		duration: u32,
	) -> DispatchResultWithPostInfo {
		let bounded_account: BoundedVec<T::AccountId, <T as Config>::AccountLimit> = Vec::new()
			.clone()
			.try_into()
			.map_err(|_| Error::<T>::AccountLimitReached)?;

		let new_bounty = Bounty {
			creator,
			name,
			category,
			badge,
			description,
			status: BountyStatus::Ongoing,
			participants: bounded_account.clone(),
			contributors: Vec::new(),
			duration,
		};

		let bounty_id =
			NextBountyId::<T>::get(community_id).unwrap_or(T::BountyId::initial_value());
		//
		// // Set up the expire time of a particular bounty with community id.
		// let total_block: u32 = BLOCKS_PER_DAY * duration;

		// Storing the Bounty
		<Bounties<T>>::insert(community_id, bounty_id, &new_bounty);

		let next_id = bounty_id.increment();
		NextBountyId::<T>::insert(community_id, next_id);

		Self::deposit_event(Event::CreatedBounty(bounty_id));

		Ok(().into())
	}
}
