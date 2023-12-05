use crate::{mock::*, Bounties, Error};
use frame_support::assert_noop;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use pallet_community::types::{
	Category, CommunityMetaData, CommunityType, Customs, Languages, Religions, Territories,
	Traditions, Values,
};
use pallet_passport::types::BadgesType;

fn get_community_metadata() -> CommunityMetaData<ConstU32<250>> {
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

pub fn add_founder() {
	Whitelist::add_founder(RuntimeOrigin::root(), 1).unwrap();
}

fn create_community() {
	Community::create_community(
		RuntimeOrigin::signed(1),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some(
			"Jur is the core community of the Jur ecosystem, which includes all the contributors."
				.into(),
		),
		Some(vec![1, 2, 3, 4, 5]),
		Some(get_community_metadata()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation),
	)
	.unwrap();
}

fn add_badge() {
	let badge_name: Vec<u8> = "DEVBOUNTY".into();
	let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

	let badge_description: Vec<u8> = "Development bounty for the jur community members".into();
	let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
		badge_description.try_into().unwrap();

	let badge_address: Vec<u8> =
		"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
	let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

	Passport::add_badge(
		RuntimeOrigin::signed(1),
		1,
		bounded_badge_name,
		BadgesType::Participation,
		bounded_badge_description,
		bounded_badge_address,
	)
	.unwrap();
}

fn mint_passport_with_badge() {
	add_founder();
	create_community();
	add_badge();
	Passport::mint(RuntimeOrigin::signed(2), 1).unwrap();
}

pub fn create_bounty() {
	mint_passport_with_badge();

	let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
	let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

	let bounty_category: Vec<u8> = "DEVELOPMENT".into();
	let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> = bounty_category.try_into().unwrap();

	let bounty_category: Vec<u8> = "DESIGN".into();
	let bounded_bounty_category2: BoundedVec<u8, ConstU32<20>> =
		bounty_category.try_into().unwrap();

	let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
	let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
		bounty_description.try_into().unwrap();

	let badge_name: Vec<u8> = "DEVBOUNTY".into();
	let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

	BountyPallet::create_bounty(
		RuntimeOrigin::signed(1),
		1,
		bounded_bounty_name,
		vec![bounded_bounty_category, bounded_bounty_category2],
		bounded_badge_name,
		bounded_bounty_description,
		2,
	)
	.unwrap();
}

fn update_bounty() {
	create_bounty();
	Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
	Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

	let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
		vec![3, 4].try_into().unwrap();

	BountyPallet::update_bounty(RuntimeOrigin::signed(1), 1, 1, bounded_accounts).unwrap();
}

#[test]
fn create_bounty_works() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> =
			bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
			bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		BountyPallet::create_bounty(
			RuntimeOrigin::signed(1),
			1,
			bounded_bounty_name,
			vec![bounded_bounty_category],
			bounded_badge_name,
			bounded_bounty_description,
			2,
		)
		.unwrap();

		assert!(Bounties::<Test>::get(1, 1).is_some());
	});
}

#[test]
fn create_bounty_not_works_invalid_community_id() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> =
			bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
			bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			BountyPallet::create_bounty(
				RuntimeOrigin::signed(1),
				5,
				bounded_bounty_name,
				vec![bounded_bounty_category],
				bounded_badge_name,
				bounded_bounty_description,
				2,
			),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn create_bounty_not_works_invalid_founder() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> =
			bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
			bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			BountyPallet::create_bounty(
				RuntimeOrigin::signed(2),
				1,
				bounded_bounty_name,
				vec![bounded_bounty_category],
				bounded_badge_name,
				bounded_bounty_description,
				2,
			),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn create_bounty_not_works_for_invalid_badge() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> =
			bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
			bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEV".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			BountyPallet::create_bounty(
				RuntimeOrigin::signed(1),
				1,
				bounded_bounty_name,
				vec![bounded_bounty_category],
				bounded_badge_name,
				bounded_bounty_description,
				2,
			),
			Error::<Test>::BadgeNotExist
		);
	});
}

#[test]
fn create_bounty_not_works_for_invalid_bounty_duration() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let bounty_name: Vec<u8> = "Bounty to help in dev work".into();
		let bounded_bounty_name: BoundedVec<u8, ConstU32<512>> = bounty_name.try_into().unwrap();

		let bounty_category: Vec<u8> = "DEVELOPMENT".into();
		let bounded_bounty_category: BoundedVec<u8, ConstU32<20>> =
			bounty_category.try_into().unwrap();

		let bounty_description: Vec<u8> = "Development bounty for the jur community members".into();
		let bounded_bounty_description: BoundedVec<u8, ConstU32<8192>> =
			bounty_description.try_into().unwrap();

		let badge_name: Vec<u8> = "DEVBOUNTY".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			BountyPallet::create_bounty(
				RuntimeOrigin::signed(1),
				1,
				bounded_bounty_name,
				vec![bounded_bounty_category],
				bounded_badge_name,
				bounded_bounty_description,
				390,
			),
			Error::<Test>::InvalidBountyDuration
		);
	});
}

#[test]
fn update_bounty_works() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![3, 4].try_into().unwrap();

		BountyPallet::update_bounty(RuntimeOrigin::signed(1), 1, 1, bounded_accounts).unwrap();
	});
}

#[test]
fn update_bounty_not_works_for_invalid_community_id() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![3, 4].try_into().unwrap();

		assert_noop!(
			BountyPallet::update_bounty(RuntimeOrigin::signed(1), 5, 1, bounded_accounts,),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_bounty_not_works_for_invalid_bounty_id() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![3, 4].try_into().unwrap();

		assert_noop!(
			BountyPallet::update_bounty(RuntimeOrigin::signed(1), 1, 2, bounded_accounts,),
			Error::<Test>::BountyNotAvailable
		);
	});
}

#[test]
fn update_bounty_not_works_for_invalid_founder() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![3, 4].try_into().unwrap();

		assert_noop!(
			BountyPallet::update_bounty(RuntimeOrigin::signed(2), 1, 2, bounded_accounts,),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn update_bounty_not_works_for_founder_as_participant() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![1, 4].try_into().unwrap();

		assert_noop!(
			BountyPallet::update_bounty(RuntimeOrigin::signed(1), 1, 2, bounded_accounts,),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn update_bounty_not_works_after_deadline() {
	new_test_ext().execute_with(|| {
		create_bounty();
		Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
		Passport::mint(RuntimeOrigin::signed(4), 1).unwrap();

		let bounded_accounts: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<500>> =
			vec![3, 4].try_into().unwrap();

		run_to_block(30000);

		assert_noop!(
			BountyPallet::update_bounty(RuntimeOrigin::signed(1), 1, 1, bounded_accounts,),
			Error::<Test>::BountyClosed
		);
	});
}

#[test]
fn complete_bounty_works() {
	new_test_ext().execute_with(|| {
		update_bounty();

		BountyPallet::complete_bounty(RuntimeOrigin::signed(1), 1, 1, vec![3]).unwrap();
	});
}

#[test]
fn complete_bounty_not_works_invalid_community_id() {
	new_test_ext().execute_with(|| {
		update_bounty();

		Passport::mint(RuntimeOrigin::signed(5), 1).unwrap();

		assert_noop!(
			BountyPallet::complete_bounty(RuntimeOrigin::signed(1), 4, 1, vec![3],),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn complete_bounty_not_works_invalid_bounty_id() {
	new_test_ext().execute_with(|| {
		update_bounty();

		Passport::mint(RuntimeOrigin::signed(5), 1).unwrap();

		assert_noop!(
			BountyPallet::complete_bounty(RuntimeOrigin::signed(1), 1, 4, vec![3],),
			Error::<Test>::BountyNotAvailable
		);
	});
}

#[test]
fn complete_bounty_not_works_invalid_participant() {
	new_test_ext().execute_with(|| {
		update_bounty();

		Passport::mint(RuntimeOrigin::signed(5), 1).unwrap();

		assert_noop!(
			BountyPallet::complete_bounty(RuntimeOrigin::signed(1), 1, 1, vec![5],),
			Error::<Test>::ParticipantNotAvailable
		);
	});
}

#[test]
fn complete_bounty_not_works_founder_as_contributor() {
	new_test_ext().execute_with(|| {
		update_bounty();

		Passport::mint(RuntimeOrigin::signed(5), 1).unwrap();

		assert_noop!(
			BountyPallet::complete_bounty(RuntimeOrigin::signed(1), 1, 1, vec![1],),
			Error::<Test>::NotAllowed
		);
	});
}
