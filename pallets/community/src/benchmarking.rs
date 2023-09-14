//! Benchmarking setup for pallet-community

#![cfg(feature = "runtime-benchmarks")]
use super::*;
#[allow(unused)]
use crate::Pallet as Community;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{prelude::*, vec};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_metadata<T: Config>() -> CommunityMetaData<T::CustomLimit> {
	let custom_one: Vec<u8> = "in public transport young people should leave the seat to elderly or pregnant women"
		.into();
	let custom_two: Vec<u8> = "name newborns with a name that starts with the letter A".into();
		CommunityMetaData {
		customs: Some(vec![
			Customs(custom_one.try_into().unwrap()),
			Customs(custom_two.try_into().unwrap()),
		]),
		languages: Some(vec!["English".into(), "German".into()]),
		norms: Some(vec![]),
		religions: Some(vec!["Christianity".into(), "Buddhism".into()]),
		territories: Some(vec!["Mars".into()]),
		traditions: Some(vec![
			"Exchange gifts for Christmas".into(),
			"Organize one charity event every 100 blocks".into(),
		]),
		values: Some(vec!["Peace".into(), "No gender discrimination".into()]),
	}
}
benchmarks! {
	create_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED), account("sub", 2, SEED)];
		pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

	}: _(
		RawOrigin::Signed(caller.clone()),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation)
	)
	verify {
		assert!(Communities::<T>::get(T::Helper::community(1)).is_some());
	}

	update_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
			Some(members),
			Some(get_metadata::<T>()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
            Some(CommunityType::Nation)
		).unwrap();

		let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
		let description = "Jur is the core community of the Jur ecosystem";

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(1), Some(logo.into()), Some(description.into())
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedCommunity(T::Helper::community(1)).into());
	}

	update_metadata {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
			Some(members),
			Some(get_metadata::<T>()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
            Some(CommunityType::Nation)
		).unwrap();

		let custom_one: Vec<u8> = "in public transport young people should leave the seat to elderly or pregnant women"
		.into();
		let custom_two: Vec<u8> = "name newborns with a name that starts with the letter A".into();
		let community_metadata = CommunityMetaData {
			customs: Some(vec![
			Customs(custom_one.try_into().unwrap()),
			Customs(custom_two.try_into().unwrap()),
		]),
			languages: Some(vec!["Spanish".into(), "Swish".into()]),
			norms: None,
			religions: Some(vec!["Christianity".into(), "Buddhism".into()]),
			territories: None,
			traditions: Some(vec![
				"Exchange gifts for Christmas".into(),
				"Organize one charity event every 100 blocks".into(),
			]),
			values: Some(vec!["Peace".into(), "No gender discrimination".into()]),
		};

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(1), community_metadata
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedMetadata(T::Helper::community(1)).into());
	}

	accept_members {
	let caller: T::AccountId = whitelisted_caller();
	let members = vec![account("sub", 1, SEED)];

	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation)
	).unwrap();

	let members = vec![account("sub", 2, SEED), account("sub", 3, SEED)];

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(1), members
	)
	verify {
		assert_last_event::<T>(Event::<T>::AddedMembers(T::Helper::community(1)).into());
	}

	join_community {
	let caller: T::AccountId = whitelisted_caller();
	let members = vec![account("sub", 1, SEED)];

	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
        Some(CommunityType::Nation)
	).unwrap();

		let member: T::AccountId = account("sub", 2, SEED);

	}: _(
		RawOrigin::Signed(member), T::Helper::community(1)
	)
	verify {
		assert_last_event::<T>(Event::<T>::JoinedCommunity(T::Helper::community(1)).into());
	}

	leave_community {
	let caller: T::AccountId = whitelisted_caller();
	let member: T::AccountId = whitelisted_caller();

	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(vec![member.clone()]),
		Some(get_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
        Some(CommunityType::Nation)
	).unwrap();

	}: _(
		RawOrigin::Signed(member), T::Helper::community(1)
	)
	verify {
		assert_last_event::<T>(Event::<T>::LeavedCommunity(T::Helper::community(1)).into());
	}

	remove_member {
	let caller: T::AccountId = whitelisted_caller();
	let member: T::AccountId = whitelisted_caller();

	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(vec![member.clone()]),
		Some(get_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation)

	).unwrap();

	}: _(
		RawOrigin::Signed(caller), member.clone(), T::Helper::community(1)
	)
	verify {
		assert_last_event::<T>(Event::<T>::RemovedMember(member).into());
	}

	update_passport_metadata {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller.clone()).unwrap();

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
			Some(members),
			Some(get_metadata::<T>()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation)
		).unwrap();

		let tag = "Alpha";
		let p_color = "#E76081";
		let s_color = "#222308";

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(1), Some(tag.into()), Some(p_color.into()), Some(s_color.into())
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedTagAndColors(T::Helper::community(1)).into());
	}

	impl_benchmark_test_suite!(Community, crate::mock::new_test_ext(), crate::mock::Test);
}
