//! # Jur Events Pallet
//!
//! A pallet allow founders to create the Events for the community members.
//!
//! ## Overview
//!
//! A Bounty Spending is a reward for a specified body of work or specified set of objectives.
//!
//! ## Interface
//!
//! * `create_event`
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{EventDetails, EventType};
use frame_support::{dispatch::DispatchResultWithPostInfo, BoundedVec};
use pallet_passport::Passports;
use primitives::Incrementable;
use sp_std::vec::Vec;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;

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
	pub trait Config:
		frame_system::Config
		+ pallet_community::Config
		+ pallet_passport::Config
		+ pallet_timestamp::Config
	{
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the Event.
		type EventId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The maximum length of Event name/title.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of Event description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		// #[cfg(feature = "runtime-benchmarks")]
		// /// A set of helper functions for benchmarking.
		// type Helper: BenchmarkHelper<Self::BountyId>;
		//
		// // Weight information
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store new Event with a unique event id for a particular community
	#[pallet::storage]
	#[pallet::getter(fn bounties)]
	pub type Events<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		T::EventId,
		EventDetails<
			<T as Config>::NameLimit,
			<T as Config>::DescriptionLimit,
			T::BadgeNameLimit,
			T::AccountId,
		>,
		OptionQuery,
	>;

	/// Stores the `EventId` that is going to be used for the next event.
	/// This gets incremented whenever a new event is created.
	#[pallet::storage]
	pub type NextEventId<T: Config> =
		StorageMap<_, Twox64Concat, T::CommunityId, T::EventId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Event [Community Id, Event Id]
		CreatedEvent(T::CommunityId, T::EventId),
		/// Issued Badge [Community Id, Event Id, Account Id]
		IssuedBadge(T::CommunityId, T::EventId, T::AccountId),
		/// member's Proof of present added to the chain.[Community Id, Event Id, Account Id]
		PopAdded(T::CommunityId, T::EventId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Community Does Not Exist.
		CommunityDoesNotExist,
		/// NotAllowed
		NotAllowed,
		/// Invalid description given.
		BadDescription,
		/// Passport not available given user.
		PassportNotAvailable,
		/// Bounty badge not exist.
		BadgeNotExist,
		/// Event time given is invalid.
		InvalidEventTime,
		/// Badge already issued to the member.
		BadgeAlreadyIssued,
		/// Event doesn't exist on the chain.
		EventDoesNotExist,
		/// Member does not exist in member pool of the community.
		MemberDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		u64: From<<T as pallet_timestamp::Config>::Moment>,
	{
		/// Create a new event for a particular community from a origin.
		///
		/// The origin must be Signed and the community founder.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `name`: name/title of the Event.
		/// - `description`: description of the Event.
		/// - `start_time`: The timestamp when the Event will start.
		/// - `end_time`: The timestamp when the Event will end.
		/// - `event_type`: Type of the event physical/online.
		/// - `venue`: Venue of the physical event.
		/// - `badge`: Badge name for the event.
		///
		/// Emits `CreatedEvent` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(1000000)]
		pub fn create_event(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			start_time: u64,
			end_time: u64,
			event_type: EventType,
			venue: Option<BoundedVec<u8, <T as pallet::Config>::NameLimit>>,
			badge: BoundedVec<u8, T::BadgeNameLimit>,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			// Ensuring the event creator should be founder.
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			// Ensuring the event badge should exist in the community badge directory.
			let is_badge_exist = pallet_passport::Badges::<T>::get(community_id, &badge);
			ensure!(is_badge_exist.is_some(), Error::<T>::BadgeNotExist);

			// Ensuring the start time is less then the end time of the event.
			ensure!(start_time < end_time, Error::<T>::InvalidEventTime);

			Self::do_create_event(
				origin,
				community_id,
				name,
				description,
				start_time,
				end_time,
				event_type,
				venue,
				badge,
			)
		}

		/// Issue Proof of presence in the event as passport badge.
		///
		/// The origin must be Signed and the community founder of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `name`: Badge name which we want to issue to members.
		/// - `members`: Member account address whom we want to issue badge
		///
		/// Emits `IssuedBadge` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(1000000)]
		pub fn proof_of_presence(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			event_id: T::EventId,
			member: T::AccountId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			// Ensuring the badge issuer should not be the founder of the community
			ensure!(origin != community.founder, Error::<T>::NotAllowed);

			// Ensure the origin should be admin.
			ensure!(
				pallet_whitelist::Admins::<T>::get()
					.binary_search(&origin)
					.is_ok(),
				Error::<T>::NotAllowed
			);

			// Ensuring the member should be the of community.
			ensure!(community.members.contains(&member), Error::<T>::MemberDoesNotExist);

			let event_data =
				<Events<T>>::get(community_id, event_id).ok_or(Error::<T>::EventDoesNotExist)?;
			let event_badge = event_data.badge;

			// Ensuring the members should have the passport and dont have the same badge
			ensure!(
				<Passports<T>>::get(community_id, member.clone()).is_some(),
				Error::<T>::PassportNotAvailable
			);

			ensure!(
				!<Passports<T>>::get(community_id, member.clone())
					.unwrap()
					.badges
					.contains(&event_badge),
				Error::<T>::BadgeAlreadyIssued
			);

			// Adding the member to the attendees list of the event.
			Events::<T>::try_mutate(
				community_id,
				event_id,
				|event_details| -> DispatchResult {
					let event = event_details
						.as_mut()
						.ok_or(Error::<T>::EventDoesNotExist)?;

					let mut attendees = event.attendees_list.clone();
					attendees.push(member.clone());
					event.attendees_list = attendees;

					Self::deposit_event(Event::PopAdded(community_id, event_id, member.clone()));

					Ok(())
				},
			)?;

			// Issuing the badge to the member
			Passports::<T>::try_mutate(
				community_id,
				member.clone(),
				|passport_details| -> DispatchResult {
					let passport = passport_details
						.as_mut()
						.ok_or(Error::<T>::PassportNotAvailable)?;

					let mut badges = passport.badges.clone();
					badges.push(event_badge.clone());
					passport.badges = badges;

					Ok(())
				},
			)?;

			Self::deposit_event(Event::IssuedBadge(community_id, event_id, member));
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_event(
		creator: T::AccountId,
		community_id: T::CommunityId,
		name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
		description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
		start_time: u64,
		end_time: u64,
		event_type: EventType,
		venue: Option<BoundedVec<u8, <T as pallet::Config>::NameLimit>>,
		badge: BoundedVec<u8, T::BadgeNameLimit>,
	) -> DispatchResultWithPostInfo {
		let event_id = NextEventId::<T>::get(community_id).unwrap_or(T::EventId::initial_value());

		let new_event = EventDetails {
			creator,
			name,
			description,
			start_time,
			end_time,
			event_type,
			venue,
			badge,
			attendees_list: Vec::new(),
		};

		// Storing the Event details
		<Events<T>>::insert(community_id, event_id, &new_event);

		let next_id = event_id.increment();
		NextEventId::<T>::insert(community_id, next_id);

		Self::deposit_event(Event::CreatedEvent(community_id, event_id));

		Ok(().into())
	}
}
