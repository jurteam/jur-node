//! Benchmarking setup for pallet-passport

use super::*;

#[allow(unused)]
use crate::Pallet as Passport;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use pallet_community::types::{CommunityMetaData, CommunityType};
use sp_std::vec;
use sp_std::vec::Vec;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_community_metadata<T: Config>() -> CommunityMetaData<T::AccountId, T::Hash> {
	CommunityMetaData {
		community_type: CommunityType::Nation,
		customs: vec![
			"in public transport young people should leave the seat to elderly or pregnant women"
				.into(),
			"name newborns with a name that starts with the letter A".into(),
		],
		languages: vec!["English".into(), "German".into()],
		norms: vec![],
		religions: vec!["Christianity".into(), "Buddhism".into()],
		territories: vec!["Mars".into()],
		traditions: vec![
			"Exchange gifts for Christmas".into(),
			"Organize one charity event every 100 blocks".into(),
		],
		values: vec!["Peace".into(), "No gender discrimination".into()],
	}
}

fn create_community<T: Config>(caller: T::AccountId) -> T::CommunityId {
	let community_id =
		pallet_community::NextCommunityId::<T>::get().unwrap_or(T::CommunityId::initial_value());

	let members = vec![account("sub", 1, SEED)];

	pallet_community::Pallet::<T>::create_community(
		RawOrigin::Signed(caller).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".as_bytes().to_vec(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors."
			.into()),
		Some(members),
		Some(get_community_metadata::<T>()),
	)
	.unwrap();

	community_id
}

benchmarks! {
	mint {
		let caller: T::AccountId = whitelisted_caller();
		let member: T::AccountId = account("sub", 1, SEED);
		let community_id = create_community::<T>(caller.clone());
	}: _(
		RawOrigin::Signed(caller.clone()),
		member,
		community_id
	)
	verify {
		assert_last_event::<T>(Event::<T>::MintedPassport(
			<T as pallet::Config>::Helper::passport(0),
			caller
		).into());
	}

	update_passport {
		let caller: T::AccountId = whitelisted_caller();
		let member: T::AccountId = account("sub", 1, SEED);
		let community_id = create_community::<T>(caller.clone());

		Passport::<T>::mint(
		RawOrigin::Signed(caller.clone()).into(),
		member.clone(),
		community_id.clone()
		).unwrap();

		let passport_address: Vec<u8> =
		"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, <T as pallet::Config>::AddressLimit> =
		passport_address.try_into().unwrap();

	}: _(RawOrigin::Signed(caller), community_id, member, bounded_passport_address)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedPassport(
			<T as pallet::Config>::Helper::passport(0)
		).into());
	}

	add_stamps {
		let caller: T::AccountId = whitelisted_caller();
		let member: T::AccountId = account("sub", 1, SEED);
		let community_id = create_community::<T>(caller.clone());

		Passport::<T>::mint(
		RawOrigin::Signed(caller.clone()).into(),
		member.clone(),
		community_id.clone()
		).unwrap();

		let stamp_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_stamp_address: BoundedVec<u8, <T as pallet::Config>::AddressLimit> =
		stamp_address.try_into().unwrap();

	}: _(RawOrigin::Signed(caller), community_id, member, bounded_stamp_address)
	verify {
		assert_last_event::<T>(Event::<T>::AddedStamp(
			<T as pallet::Config>::Helper::passport(0)
		).into());
	}

	update_avatar {
		let caller: T::AccountId = whitelisted_caller();
		let member: T::AccountId = account("sub", 1, SEED);
		let community_id = create_community::<T>(caller.clone());

		Passport::<T>::mint(
		RawOrigin::Signed(caller.clone()).into(),
		member.clone(),
		community_id.clone()
		).unwrap();

		let avatar_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_avatar_address: BoundedVec<u8, <T as pallet::Config>::AddressLimit> =
		avatar_address.try_into().unwrap();

	}: _(RawOrigin::Signed(member), community_id, bounded_avatar_address)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedAvatar(
			<T as pallet::Config>::Helper::passport(0)
		).into());
	}

	impl_benchmark_test_suite!(Passport, crate::mock::new_test_ext(), crate::mock::Test);
}
