use crate::{mock::*, types::BadgesType, Error, Passports};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};
use pallet_community::types::{Category, CommunityMetaData, CommunityType};

fn get_community_metadata() -> CommunityMetaData<u64> {
	let community_metadata = CommunityMetaData {
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
		Some(vec![1, 2]),
		Some(get_community_metadata()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
	)
	.unwrap();
}

fn mint_passport() {
	add_founder();
	create_community();
	Passport::mint(RuntimeOrigin::signed(2), 1).unwrap();
}

#[test]
fn mint_passport_works_for_founder() {
	new_test_ext().execute_with(|| {
		add_founder();
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(1), 1));
		assert_eq!(Passports::<Test>::get(1, 1).unwrap().id, 5035);
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(1), 2));
		assert_eq!(Passports::<Test>::get(2, 1).unwrap().id, 1);
	});
}

#[test]
fn mint_passport_works_for_member() {
	new_test_ext().execute_with(|| {
		add_founder();
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(2), 1));

		assert!(Passports::<Test>::get(1, 2).is_some());
	});
}

#[test]
fn mint_passport_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		add_founder();
		create_community();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(2), 2),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn mint_passport_not_works_when_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		add_founder();
		create_community();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(12), 1),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn mint_passport_not_works_when_passport_already_minted() {
	new_test_ext().execute_with(|| {
		mint_passport();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(2), 1),
			Error::<Test>::PassportAlreadyMinted
		);
	});
}

#[test]
fn update_passport_works() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_ok!(Passport::update_passport(
			RuntimeOrigin::signed(2),
			1,
			bounded_passport_address.clone()
		));

		assert_eq!(
			Passports::<Test>::get(1, 2).unwrap().address.unwrap(),
			bounded_passport_address
		);
	});
}

#[test]
fn update_passport_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_noop!(
			Passport::update_passport(RuntimeOrigin::signed(1), 5, bounded_passport_address),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_passport_not_works_for_invalid_member() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_noop!(
			Passport::update_passport(RuntimeOrigin::signed(3), 5, bounded_passport_address),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_passport_not_works_for_unminted_passport() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_noop!(
			Passport::update_passport(RuntimeOrigin::signed(1), 1, bounded_passport_address),
			Error::<Test>::PassportNotAvailable
		);
	});
}

#[test]
fn add_badge() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name,
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));
	});
}

#[test]
fn add_badge_not_work_for_invalid_community() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_noop!(
			Passport::add_badge(
				RuntimeOrigin::signed(1),
				2,
				bounded_badge_name,
				BadgesType::Participation,
				bounded_badge_description,
				bounded_badge_address
			),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn add_badge_not_work_for_invalid_founder() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_noop!(
			Passport::add_badge(
				RuntimeOrigin::signed(2),
				1,
				bounded_badge_name,
				BadgesType::Participation,
				bounded_badge_description,
				bounded_badge_address
			),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn add_badge_not_work_for_badge_already_exist() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description.clone(),
			bounded_badge_address.clone()
		));

		assert_noop!(
			Passport::add_badge(
				RuntimeOrigin::signed(1),
				1,
				bounded_badge_name,
				BadgesType::Participation,
				bounded_badge_description,
				bounded_badge_address
			),
			Error::<Test>::BadgeAlreadyExist
		);
	});
}

#[test]
fn issue_badge() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));

		assert_ok!(Passport::issue_badge(RuntimeOrigin::signed(1), 1, bounded_badge_name, vec![2]));
	});
}

#[test]
fn issue_badge_not_work_invalid_community() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));

		assert_noop!(
			Passport::issue_badge(RuntimeOrigin::signed(1), 2, bounded_badge_name, vec![2]),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn issue_badge_not_work_invalid_founder() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));

		assert_noop!(
			Passport::issue_badge(RuntimeOrigin::signed(2), 1, bounded_badge_name, vec![2]),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn issue_badge_not_work_invalid_badge_name() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));

		let badge_name: Vec<u8> = "JUR Meet".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			Passport::issue_badge(RuntimeOrigin::signed(1), 1, bounded_badge_name, vec![2]),
			Error::<Test>::BadgeNotAvailable
		);
	});
}

#[test]
fn issue_badge_not_work_invalid_community_member() {
	new_test_ext().execute_with(|| {
		mint_passport();

		let badge_name: Vec<u8> = "JUR Meetup".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		let badge_description: Vec<u8> =
			"JUR Meetup is the get together time for the jur community".into();
		let bounded_badge_description: BoundedVec<u8, ConstU32<250>> =
			badge_description.try_into().unwrap();

		let badge_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_badge_address: BoundedVec<u8, ConstU32<60>> = badge_address.try_into().unwrap();

		assert_ok!(Passport::add_badge(
			RuntimeOrigin::signed(1),
			1,
			bounded_badge_name.clone(),
			BadgesType::Participation,
			bounded_badge_description,
			bounded_badge_address
		));

		assert_noop!(
			Passport::issue_badge(RuntimeOrigin::signed(1), 1, bounded_badge_name, vec![4]),
			Error::<Test>::PassportNotAvailable
		);
	});
}
