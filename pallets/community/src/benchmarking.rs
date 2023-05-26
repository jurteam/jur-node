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

fn get_metadata<T: Config>() -> CommunityMetaData<T::AccountId, T::Hash> {
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
benchmarks! {
	create_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED), account("sub", 2, SEED)];

	}: _(
		RawOrigin::Signed(caller.clone()),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		"Jur is the core community of the Jur ecosystem, which includes all the contributors.".into(),
		members,
		Some(get_metadata::<T>())
	)
	verify {
		assert_last_event::<T>(Event::<T>::CreatedCommunity(T::Helper::community(0), caller).into());
	}

	delete_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			"Jur is the core community of the Jur ecosystem, which includes all the contributors.".into(),
			members,
			Some(get_metadata::<T>())
		).unwrap();

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(0)
	)
	verify {
		assert_last_event::<T>(Event::<T>::DeletedCommunity(T::Helper::community(0)).into());
	}

	update_community {
		let caller: T::AccountId = whitelisted_caller();
		let members = vec![account("sub", 1, SEED)];

		Community::<T>::create_community(
			RawOrigin::Signed(caller.clone()).into(),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur".into(),
			"Jur is the core community of the Jur ecosystem, which includes all the contributors.".into(),
			members,
			Some(get_metadata::<T>())
		).unwrap();

		let metadata: CommunityMetaData<T::AccountId, T::Hash> = CommunityMetaData {
			community_type: CommunityType::Nation,
			customs: vec![
				"in public transport young people should leave the seat to elderly or pregnant women"
					.into(),
				"name newborns with a name that starts with the letter A".into(),
			],
			languages: vec!["English".into()],
			norms: vec![],
			religions: vec!["Christianity".into()],
			territories: vec!["Mars".into()],
			traditions: vec![
				"Exchange gifts for Christmas".into(),
				"Organize one charity event every 100 blocks".into(),
			],
			values: vec!["Peace".into(), "No gender discrimination".into()],
		};

		let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
		let description = "Jur is the core community of the Jur ecosystem";

	}: _(
		RawOrigin::Signed(caller), Some(logo.into()), description.into(), T::Helper::community(0), metadata
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedCommunity(T::Helper::community(0)).into());
	}

	add_members {
	let caller: T::AccountId = whitelisted_caller();
	let members = vec![account("sub", 1, SEED)];

	Community::<T>::create_community(
		RawOrigin::Signed(caller.clone()).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		"Jur is the core community of the Jur ecosystem, which includes all the contributors.".into(),
		members,
		Some(get_metadata::<T>())
	).unwrap();

	let members = vec![account("sub", 2, SEED), account("sub", 3, SEED)];

	}: _(
		RawOrigin::Signed(caller), T::Helper::community(0), members
	)
	verify {
		assert_last_event::<T>(Event::<T>::AddedMembers(T::Helper::community(0)).into());
	}

	impl_benchmark_test_suite!(Community, crate::mock::new_test_ext(), crate::mock::Test);
}
