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

fn get_metadata<T: Config>() -> CommunityMetaData<T::AccountId,> {
	CommunityMetaData {
		community_type: Some(CommunityType::Nation),
		customs: Some(vec![
			"in public transport young people should leave the seat to elderly or pregnant women"
				.into(),
			"name newborns with a name that starts with the letter A".into(),
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

	}: _(
		RawOrigin::Signed(caller.clone()),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>())
	)
	verify {
		assert_last_event::<T>(Event::<T>::CreatedCommunity(T::Helper::community(0), caller).into());
	}

	update_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
			Some(members),
			Some(get_metadata::<T>())
		).unwrap();

		let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
		let description = "Jur is the core community of the Jur ecosystem";

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(0), Some(logo.into()), Some(description.into())
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedCommunity(T::Helper::community(0)).into());
	}

	update_metadata {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
			Some(members),
			Some(get_metadata::<T>())
		).unwrap();

		let community_metadata = CommunityMetaData {
			community_type: Some(CommunityType::Nation),
			customs: Some(vec![
				"in public transport young people should leave the seat to elderly or pregnant women"
					.into(),
				"name newborns with a name that starts with the letter A".into(),
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
		RawOrigin::Signed(caller), T::Helper::community(0), community_metadata
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedMetadata(T::Helper::community(0)).into());
	}

	add_members {
	let caller: T::AccountId = whitelisted_caller();
	let members = vec![account("sub", 1, SEED)];

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>())
	).unwrap();

	let members = vec![account("sub", 2, SEED), account("sub", 3, SEED)];

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(0), members
	)
	verify {
		assert_last_event::<T>(Event::<T>::AddedMembers(T::Helper::community(0)).into());
	}

	join_community {
	let caller: T::AccountId = whitelisted_caller();
	let members = vec![account("sub", 1, SEED)];

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(members),
		Some(get_metadata::<T>())
	).unwrap();

		let member: T::AccountId = whitelisted_caller();

	}: _(
		RawOrigin::Signed(member), T::Helper::community(0)
	)
	verify {
		assert_last_event::<T>(Event::<T>::JoinedCommunity(T::Helper::community(0)).into());
	}

	leave_community {
	let caller: T::AccountId = whitelisted_caller();
	let member: T::AccountId = whitelisted_caller();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(vec![member.clone()]),
		Some(get_metadata::<T>())
	).unwrap();

	}: _(
		RawOrigin::Signed(member), T::Helper::community(0)
	)
	verify {
		assert_last_event::<T>(Event::<T>::LeavedCommunity(T::Helper::community(0)).into());
	}

	remove_member {
	let caller: T::AccountId = whitelisted_caller();
	let member: T::AccountId = whitelisted_caller();

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors.".into()),
		Some(vec![member.clone()]),
		Some(get_metadata::<T>())
	).unwrap();

	}: _(
		RawOrigin::Signed(caller), member.clone(), T::Helper::community(0)
	)
	verify {
		assert_last_event::<T>(Event::<T>::RemovedMember(member).into());
	}

	impl_benchmark_test_suite!(Community, crate::mock::new_test_ext(), crate::mock::Test);
}
