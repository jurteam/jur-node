//! # Jur Passport Pallet
//!
//! A pallet allows a meta-citizen to mint its own Jur NFT Passport.
//! This passport is what allows access to different features within the ecosystem.
//!
//! ## Overview
//!
//! A Passport is an official document released by a State that proves
//! that you are a citizen of that State.
//!
//! In Jur, this would be represented as a mintable NFT that any citizen of a State can redeem.
//! You won’t be able to mint the NFT in case
//! your wallet is not part of the citizens property of the State.
//!
//!
//! ## Interface
//!
//! * `mint`
//! * `update_passport`
//! * `add_badge`
//! * `issue_badge`
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{BadgeDetails, BadgesType, PassportDetails};
use primitives::Incrementable;
use sp_std::vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod migration;
pub mod weights;
pub use weights::WeightInfo;

const LOG_TARGET: &str = "runtime::passport";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<PassportId> {
		fn passport(i: u32) -> PassportId;
	}
	#[cfg(feature = "runtime-benchmarks")]
	impl<PassportId: From<u32>> BenchmarkHelper<PassportId> for () {
		fn passport(i: u32) -> PassportId {
			i.into()
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_community::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the Passport.
		type PassportId: Member + Parameter + MaxEncodedLen + Copy + Incrementable + PartialOrd;

		/// The maximum length of address.
		#[pallet::constant]
		type BadgeNameLimit: Get<u32>;

		/// The maximum length of address.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		/// The maximum length of address.
		#[pallet::constant]
		type AddressLimit: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type Helper: BenchmarkHelper<Self::PassportId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Store passport metadata for a passport holder that belongs to a particular community
	#[pallet::storage]
	#[pallet::getter(fn passport)]
	pub type Passports<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		T::AccountId,
		PassportDetails<T::PassportId, T::BadgeNameLimit, T::AddressLimit>,
		OptionQuery,
	>;

	/// Store passport metadata for a passport holder that belongs to a particular community
	#[pallet::storage]
	#[pallet::getter(fn badges)]
	pub type Badges<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		BoundedVec<u8, T::BadgeNameLimit>,
		BadgeDetails<<T as pallet::Config>::DescriptionLimit, T::AddressLimit>,
		OptionQuery,
	>;

	/// Stores the `PassportId` that is going to be used for the next passport.
	/// This gets incremented whenever a new passport is created.
	#[pallet::storage]
	pub type NextPassportId<T: Config> =
		StorageMap<_, Twox64Concat, T::CommunityId, T::PassportId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Minted Passport [passport]
		MintedPassport(T::PassportId),
		/// Updated Passport [passport]
		UpdatedPassport(T::PassportId),
		/// Badge Added
		AddedBadge(Vec<u8>),
		/// Issued Badge
		IssuedBadge(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Community Does Not Exist.
		CommunityDoesNotExist,
		/// NotAllowed
		NotAllowed,
		/// MemberDoesNotExist
		MemberDoesNotExist,
		/// PassportAlreadyMinted
		PassportAlreadyMinted,
		/// PassportNotAvailable
		PassportNotAvailable,
		/// Badge not available in badge directory.
		BadgeNotAvailable,
		/// Badge already exist in badge directory.
		BadgeAlreadyExist,
		/// Badge already Issued to the user.
		BadgeAlreadyIssued,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mint a new passport
		///
		/// The origin must be Signed and the founder/member of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		///
		/// Emits `MintedPassport` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::mint())]
		pub fn mint(origin: OriginFor<T>, community_id: T::CommunityId) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(
				origin == community.founder || community.members.contains(&origin),
				Error::<T>::MemberDoesNotExist
			);

			let maybe_passport = Passports::<T>::get(community_id, &origin);
			ensure!(maybe_passport.is_none(), Error::<T>::PassportAlreadyMinted);

			let mut passport_id =
				NextPassportId::<T>::get(community_id).unwrap_or(T::PassportId::initial_value());

			// Adding this check to reserve the slots for community
			if community_id == T::CommunityId::initial_value()
				&& passport_id < T::PassportId::jur_community_reserve_slots()
			{
				passport_id = T::PassportId::jur_community_reserve_slots();
			}

			let passport_details =
				PassportDetails { id: passport_id, address: None, badges: vec![] };

			<Passports<T>>::insert(community_id, &origin, passport_details);

			let next_id = passport_id.increment();
			NextPassportId::<T>::insert(community_id, next_id);

			Self::deposit_event(Event::MintedPassport(passport_id));
			Ok(())
		}

		/// Update the passport.
		///
		/// The origin must be Signed and the community member of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `passport_address`: IPFS Address of the passport
		///
		/// Emits `UpdatedPassport` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_passport())]
		pub fn update_passport(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			passport_address: BoundedVec<u8, T::AddressLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(
				origin == community.founder || community.members.contains(&origin),
				Error::<T>::MemberDoesNotExist
			);

			<Passports<T>>::get(community_id, &origin).ok_or(Error::<T>::PassportNotAvailable)?;

			Passports::<T>::try_mutate(community_id, origin, |passport_details| {
				let passport = passport_details
					.as_mut()
					.ok_or(Error::<T>::PassportNotAvailable)?;

				passport.address = Some(passport_address);

				Self::deposit_event(Event::UpdatedPassport(passport.id));
				Ok(())
			})
		}

		/// Add badge to the community badge directory.
		///
		/// The origin must be Signed and the founder of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `name`: name of the badge you want to add in the directory.
		/// - `badge_type`: type of badge founder wants to add in directory.
		/// - `description`: Detailed description of the badge.
		/// - `address`: IPFS address of the badge.
		///
		/// Emits `AddedBadge` event when successful.
		///
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::add_badge())]
		pub fn add_badge(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: BoundedVec<u8, T::BadgeNameLimit>,
			badge_type: BadgesType,
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			address: BoundedVec<u8, T::AddressLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			let maybe_badge = Badges::<T>::get(community_id, &name);
			ensure!(maybe_badge.is_none(), Error::<T>::BadgeAlreadyExist);

			let badge_details = BadgeDetails { badge_type, description, address };

			<Badges<T>>::insert(community_id, &name, badge_details);

			Self::deposit_event(Event::AddedBadge(name.to_vec()));
			Ok(())
		}

		/// Issue the badge to the members.
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
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::issue_badge())]
		pub fn issue_badge(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: BoundedVec<u8, T::BadgeNameLimit>,
			members: Vec<T::AccountId>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			// Ensuring the badge issuer should be the founder of the community
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			// checking the badge is available in the badge directory or not
			<Badges<T>>::get(community_id, &name).ok_or(Error::<T>::BadgeNotAvailable)?;

			// Ensuring the members should have the passport and dont have the same badge
			ensure!(
				members
					.iter()
					.find(|member| <Passports<T>>::get(community_id, member).is_none())
					.is_none(),
				Error::<T>::PassportNotAvailable
			);

			ensure!(
				members
					.iter()
					.find(|member| {
						<Passports<T>>::get(community_id, member)
							.unwrap()
							.badges
							.contains(&name)
					})
					.is_none(),
				Error::<T>::BadgeAlreadyIssued
			);

			// Issuing the badge to the members
			for member in members {
				Passports::<T>::try_mutate(
					community_id,
					member,
					|passport_details| -> DispatchResult {
						let passport = passport_details
							.as_mut()
							.ok_or(Error::<T>::PassportNotAvailable)?;

						let mut badges = passport.badges.clone();

						if !badges.contains(&name) {
							badges.push(name.clone());
							passport.badges = badges;
						}

						Ok(())
					},
				)?;
			}

			Self::deposit_event(Event::IssuedBadge(name.to_vec()));
			Ok(())
		}
	}
}
