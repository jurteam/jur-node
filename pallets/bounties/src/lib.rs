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
	pub trait Config: frame_system::Config + pallet_community::Config {
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
			<T as pallet::Config>::NameLimit,
			<T as pallet::Config>::CategoryLimit,
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
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			duration: u32,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			ensure!(
				(1..=BOUNTY_DURATION_LIMIT).contains(&duration),
				Error::<T>::InvalidBountyDuration
			);

			Self::do_create_bounty(
				origin,
				community_id,
				name,
				category,
				description,
				duration,
			)
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_bounty(
		creator: T::AccountId,
		community_id: T::CommunityId,
		name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
		category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit>,
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
			description,
			status: BountyStatus::Ongoing,
			participants: bounded_account.clone(),
			contributors: bounded_account.clone(),
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
