//! Benchmarking setup for pallet-bounty

use super::*;

use pallet_passport::types::BadgesType;
use crate::Pallet;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use pallet_community::types::{
	Category, CommunityMetaData, CommunityType, Customs, Languages, Religions, Territories,
	Traditions, Values,
};
use sp_std::vec;
use sp_std::vec::Vec;
use sp_runtime::traits::ConstU32;
const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_community_metadata<T: Config>() -> CommunityMetaData<T::StringLimit> {
	let custom_one: Vec<u8> =
		"in public transport young people should leave the seat to elderly or pregnant women"
			.into();
	let custom_two: Vec<u8> = "name newborns with a name that starts with the letter A".into();

	let languages_1: Vec<u8> = "English".into();
	let languages_2: Vec<u8> = "German".into();

	let religions_1: Vec<u8> = "Christianity".into();
	let religions_2: Vec<u8> = "Buddhism".into();

	let territories: Vec<u8> = "Mars".into();

	let traditions_1: Vec<u8> = "Exchange gifts for Christmas".into();
	let traditions_2: Vec<u8> = "Organize one charity event every 100 blocks".into();

	let values_1: Vec<u8> = "Peace".into();
	let values_2: Vec<u8> = "No gender discrimination".into();

	let community_metadata = CommunityMetaData {
		customs: Some(vec![
			Customs(custom_one.try_into().unwrap()),
			Customs(custom_two.try_into().unwrap()),
		]),
		languages: Some(vec![
			Languages(languages_1.try_into().unwrap()),
			Languages(languages_2.try_into().unwrap()),
		]),
		norms: Some(vec![]),
		religions: Some(vec![
			Religions(religions_1.try_into().unwrap()),
			Religions(religions_2.try_into().unwrap()),
		]),
		territories: Some(vec![Territories(territories.try_into().unwrap())]),
		traditions: Some(vec![
			Traditions(traditions_1.try_into().unwrap()),
			Traditions(traditions_2.try_into().unwrap()),
		]),
		values: Some(vec![
			Values(values_1.try_into().unwrap()),
			Values(values_2.try_into().unwrap()),
		]),
	};

	community_metadata
}

fn create_community<T: Config>(caller: T::AccountId) -> T::CommunityId {
	let community_id =
		pallet_community::NextCommunityId::<T>::get().unwrap_or(T::CommunityId::initial_value());

	let members = vec![account("sub", 1, SEED), account("sub", 2, SEED), account("sub", 3, SEED)];

	pallet_community::Pallet::<T>::create_community(
		RawOrigin::Signed(caller).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".as_bytes().to_vec(),
		Some(
			"Jur is the core community of the Jur ecosystem, which includes all the contributors."
				.into(),
		),
		Some(members),
		Some(get_community_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation),
	)
	.unwrap();

	community_id
}

pub fn add_founder<T: Config>(caller: T::AccountId) {
	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller).unwrap();
}

fn add_badge<T: Config>(caller: T::AccountId, community_id: T::CommunityId) {
	let badge_name: Vec<u8> = "DEVBOUNTY".into();
	let bounded_badge_name: BoundedVec<u8, <T as pallet_passport::Config>::BadgeNameLimit> = badge_name.try_into().unwrap();

	let badge_description: Vec<u8> =
		"Development bounty for the jur community members".into();
	let bounded_badge_description: BoundedVec<u8, <T as pallet_passport::Config>::DescriptionLimit> =
		badge_description.try_into().unwrap();

	let badge_address: Vec<u8> =
		"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
	let bounded_badge_address: BoundedVec<u8, <T as pallet_passport::Config>::AddressLimit> = badge_address.try_into().unwrap();

	pallet_passport::Pallet::<T>::add_badge(
		RawOrigin::Signed(caller).into(),
		community_id,
		bounded_badge_name,
		BadgesType::Participation,
		bounded_badge_description,
		bounded_badge_address,
	)
		.unwrap();
}

benchmarks! {
	create_bounty {
		let caller: T::AccountId = whitelisted_caller();
		let member = account("sub", 1, SEED);
		add_founder::<T>(caller.clone());
		let community_id = create_community::<T>(caller.clone());
		pallet_passport::Pallet::<T>::mint(RawOrigin::Signed(member).into(), community_id.clone()).unwrap();
		add_badge::<T>(caller.clone(), community_id.clone());

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit> = bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> = bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, <T as pallet_passport::Config>::BadgeNameLimit> = badge_name.try_into().unwrap();

	}: _(
		RawOrigin::Signed(caller.clone()),
		community_id,
		bounded_bounty_name,
		bounded_bounty_category,
		bounded_badge_name,
		bounded_bounty_description,
		2
	)
	verify {
		assert_last_event::<T>(Event::<T>::CreatedBounty(<T as pallet::Config>::Helper::bounty(1)).into());
	}

	update_bounty {
		let caller: T::AccountId = whitelisted_caller();
		let first_member: T::AccountId = account("sub", 2, SEED);
		let second_member: T::AccountId = account("sub", 3, SEED);
		let members = vec![account("sub", 2, SEED), account("sub", 3, SEED)];
		add_founder::<T>(caller.clone());
		let community_id = create_community::<T>(caller.clone());
		pallet_passport::Pallet::<T>::mint(RawOrigin::Signed(first_member).into(), community_id.clone()).unwrap();
		pallet_passport::Pallet::<T>::mint(RawOrigin::Signed(second_member).into(), community_id.clone()).unwrap();
		add_badge::<T>(caller.clone(), community_id.clone());
		let bounty_id = <T as pallet::Config>::Helper::bounty(1);

		let bounded_accounts: BoundedVec<<T as frame_system::Config>::AccountId, <T as pallet::Config>::AccountLimit> = members
			.try_into().unwrap();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit> = bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> = bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, <T as pallet_passport::Config>::BadgeNameLimit> = badge_name.try_into().unwrap();

		Pallet::<T>::create_bounty(
			RawOrigin::Signed(caller.clone()).into(),
			community_id,
			bounded_bounty_name,
			bounded_bounty_category,
			bounded_badge_name,
			bounded_bounty_description,
			2
		).unwrap();

	}: _(
		RawOrigin::Signed(caller.clone()),
		community_id,
		bounty_id,
		bounded_accounts
	)
	verify {
		assert_last_event::<T>(Event::<T>::UpdatedBounty(<T as pallet::Config>::Helper::bounty(1)).into());
	}

	complete_bounty {
		let caller: T::AccountId = whitelisted_caller();
		let first_member: T::AccountId = account("sub", 2, SEED);
		let second_member: T::AccountId = account("sub", 3, SEED);
		let members = vec![account("sub", 2, SEED), account("sub", 3, SEED)];
		add_founder::<T>(caller.clone());
		let community_id = create_community::<T>(caller.clone());
		pallet_passport::Pallet::<T>::mint(RawOrigin::Signed(first_member).into(), community_id.clone()).unwrap();
		pallet_passport::Pallet::<T>::mint(RawOrigin::Signed(second_member).into(), community_id.clone()).unwrap();
		add_badge::<T>(caller.clone(), community_id.clone());
		let bounty_id = <T as pallet::Config>::Helper::bounty(1);

		let bounded_accounts: BoundedVec<<T as frame_system::Config>::AccountId, <T as pallet::Config>::AccountLimit> = members.clone()
			.try_into().unwrap();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, <T as pallet::Config>::CategoryLimit> = bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> = bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, <T as pallet_passport::Config>::BadgeNameLimit> = badge_name.try_into().unwrap();

		Pallet::<T>::create_bounty(
			RawOrigin::Signed(caller.clone()).into(),
			community_id,
			bounded_bounty_name,
			bounded_bounty_category,
			bounded_badge_name,
			bounded_bounty_description,
			2
		).unwrap();

		Pallet::<T>::update_bounty(
			RawOrigin::Signed(caller.clone()).into(),
			community_id,
			bounty_id,
			bounded_accounts
		).unwrap();

	}: _(
		RawOrigin::Signed(caller.clone()),
		community_id,
		bounty_id,
		members
	)
	verify {
		assert_last_event::<T>(Event::<T>::CompletedBounty(<T as pallet::Config>::Helper::bounty(1)).into());
	}

	impl_benchmark_test_suite!(BountyPallet, crate::mock::new_test_ext(), crate::mock::Test);
}
