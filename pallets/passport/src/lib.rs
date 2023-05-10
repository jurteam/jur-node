#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::PassportDetails;
use primitives::Incrementable;
use sp_std::vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

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
		type PassportId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The maximum length of address.
		#[pallet::constant]
		type AddressLimit: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type Helper: BenchmarkHelper<Self::PassportId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn passport)]
	pub type Passports<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		T::AccountId,
		PassportDetails<T::PassportId, T::AddressLimit>,
		OptionQuery,
	>;

	/// Stores the `PassportId` that is going to be used for the next passport.
	/// This gets incremented whenever a new passport is created.
	#[pallet::storage]
	pub type NextPassportId<T: Config> = StorageValue<_, T::PassportId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Minted Passport [passport, account]
		MintedPassport(T::PassportId, T::AccountId),
		/// Updated Passport [passport]
		UpdatedPassport(T::PassportId),
		/// Added Stamp to passport [passport]
		AddedStamp(T::PassportId),
		/// Updated Avatar to passport [passport]
		UpdatedAvatar(T::PassportId),
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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mint a new passport
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::mint())]
		pub fn mint(
			origin: OriginFor<T>,
			member: T::AccountId,
			community_id: T::CommunityId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			ensure!(community.members.contains(&member), Error::<T>::MemberDoesNotExist);

			let maybe_passport = Passports::<T>::get(community_id, &member);
			ensure!(maybe_passport.is_some() == false, Error::<T>::PassportAlreadyMinted);

			let passport_id = NextPassportId::<T>::get().unwrap_or(T::PassportId::initial_value());

			let passport_details =
				PassportDetails { id: passport_id, address: None, stamps: None, avatar: None };

			<Passports<T>>::insert(community_id, &member, passport_details);

			let next_id = passport_id.increment();
			NextPassportId::<T>::set(Some(next_id));

			Self::deposit_event(Event::MintedPassport(passport_id, origin));
			Ok(())
		}

		/// Update the passport.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_passport())]
		pub fn update_passport(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			member: T::AccountId,
			passport_address: BoundedVec<u8, T::AddressLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			<Passports<T>>::get(community_id, &member).ok_or(Error::<T>::PassportNotAvailable)?;

			Passports::<T>::try_mutate(community_id, member, |passport_details| {
				let passport = passport_details
					.as_mut()
					.ok_or(Error::<T>::PassportNotAvailable)?;

				passport.address = Some(passport_address);

				Self::deposit_event(Event::UpdatedPassport(passport.id));
				Ok(())
			})
		}

		/// Add the stamp to the passport.
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::add_stamps())]
		pub fn add_stamps(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			member: T::AccountId,
			stamp: BoundedVec<u8, T::AddressLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			<Passports<T>>::get(community_id, &member).ok_or(Error::<T>::PassportNotAvailable)?;

			Passports::<T>::try_mutate(community_id, member, |passport_details| {
				let passport = passport_details
					.as_mut()
					.ok_or(Error::<T>::PassportNotAvailable)?;

				let mut stamps = vec![];

				if passport.stamps.is_some() {
					stamps = passport.stamps.clone().unwrap();
				}

				stamps.push(stamp);

				passport.stamps = Some(stamps);

				Self::deposit_event(Event::AddedStamp(passport.id));
				Ok(())
			})
		}

		/// Add/update the avatar to the passport.
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::update_avatar())]
		pub fn update_avatar(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			avatar: BoundedVec<u8, T::AddressLimit>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			ensure!(community.members.contains(&origin), Error::<T>::NotAllowed);

			<Passports<T>>::get(community_id, &origin).ok_or(Error::<T>::PassportNotAvailable)?;

			Passports::<T>::try_mutate(community_id, &origin, |passport_details| {
				let passport = passport_details
					.as_mut()
					.ok_or(Error::<T>::PassportNotAvailable)?;

				passport.avatar = Some(avatar);

				Self::deposit_event(Event::UpdatedAvatar(passport.id));
				Ok(())
			})
		}
	}
}
