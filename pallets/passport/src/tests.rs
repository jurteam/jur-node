use crate::{mock::*, Error, Passports};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};
use pallet_community::types::{CommunityMetaData, CommunityType, Category};

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

fn create_community() {
	Community::create_community(
		RuntimeOrigin::signed(1),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors."
			.into()),
		Some(vec![1, 2]),
		Some(get_community_metadata()),
		Category::Public,
		Some("tag".into()),
		Some("primary color".into()),
		Some("Secondary color".into())
	)
	.unwrap();
}

fn mint_passport() {
	create_community();
	Passport::mint(RuntimeOrigin::signed(2), 0).unwrap();
}

#[test]
fn mint_passport_works_for_founder() {
	new_test_ext().execute_with(|| {
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(1), 0));
		assert_eq!(Passports::<Test>::get(0, 1).unwrap().id, 0);
		assert_ok!(Passport::mint(RuntimeOrigin::signed(2), 0));
		assert_eq!(Passports::<Test>::get(0, 2).unwrap().id, 1);
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(1), 1));
		assert_eq!(Passports::<Test>::get(1, 1).unwrap().id, 0);
		assert_ok!(Passport::mint(RuntimeOrigin::signed(2), 1));
		assert_eq!(Passports::<Test>::get(1, 2).unwrap().id, 1);
	});
}

#[test]
fn mint_passport_works_for_member() {
	new_test_ext().execute_with(|| {
		create_community();
		assert_ok!(Passport::mint(RuntimeOrigin::signed(2), 0));

		assert!(Passports::<Test>::get(0, 2).is_some());
	});
}

#[test]
fn mint_passport_not_works_if_user_is_not_founder() {
	new_test_ext().execute_with(|| {
		create_community();
		assert_noop!(Passport::mint(RuntimeOrigin::signed(12), 0), Error::<Test>::MemberDoesNotExist);
	});
}

#[test]
fn mint_passport_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		create_community();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(2), 1),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn mint_passport_not_works_when_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		create_community();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(12), 0),
			Error::<Test>::MemberDoesNotExist
		);
	});
}

#[test]
fn mint_passport_not_works_when_passport_already_minted() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();
		assert_noop!(
			Passport::mint(RuntimeOrigin::signed(2), 0),
			Error::<Test>::PassportAlreadyMinted
		);
	});
}

#[test]
fn update_passport_works() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_ok!(Passport::update_passport(
			RuntimeOrigin::signed(1),
			0,
			2,
			bounded_passport_address.clone()
		));

		assert_eq!(
			Passports::<Test>::get(0, 2).unwrap().address.unwrap(),
			bounded_passport_address
		);
	});
}

#[test]
fn update_passport_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_noop!(
			Passport::update_passport(RuntimeOrigin::signed(1), 5, 2, bounded_passport_address),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_passport_not_works_for_unminted_passport() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let passport_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_passport_address: BoundedVec<u8, ConstU32<60>> =
			passport_address.try_into().unwrap();

		assert_noop!(
			Passport::update_passport(RuntimeOrigin::signed(1), 1, 3, bounded_passport_address),
			Error::<Test>::PassportNotAvailable
		);
	});
}

#[test]
fn update_stamps_works() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let stamp: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_stamp: BoundedVec<u8, ConstU32<60>> = stamp.try_into().unwrap();

		assert_ok!(Passport::add_stamps(RuntimeOrigin::signed(1), 0, 2, bounded_stamp.clone()));

		assert!(Passports::<Test>::get(0, 2)
			.unwrap()
			.stamps
			.unwrap()
			.contains(&bounded_stamp));
	});
}

#[test]
fn update_stamps_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let stamp: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_stamp: BoundedVec<u8, ConstU32<60>> = stamp.try_into().unwrap();

		assert_noop!(
			Passport::add_stamps(RuntimeOrigin::signed(1), 5, 2, bounded_stamp),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_stamps_not_works_for_unminted_passport() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let stamp: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_stamp: BoundedVec<u8, ConstU32<60>> = stamp.try_into().unwrap();

		assert_noop!(
			Passport::add_stamps(RuntimeOrigin::signed(1), 1, 3, bounded_stamp),
			Error::<Test>::PassportNotAvailable
		);
	});
}

#[test]
fn update_avatar_works() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let avatar: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_avatar: BoundedVec<u8, ConstU32<60>> = avatar.try_into().unwrap();

		assert_ok!(Passport::update_avatar(RuntimeOrigin::signed(2), 0, bounded_avatar.clone()));

		assert_eq!(Passports::<Test>::get(0, 2).unwrap().avatar.unwrap(), bounded_avatar);
	});
}

#[test]
fn update_avatar_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let avatar: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_avatar: BoundedVec<u8, ConstU32<60>> = avatar.try_into().unwrap();

		assert_noop!(
			Passport::update_avatar(RuntimeOrigin::signed(2), 5, bounded_avatar),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn update_avatar_not_works_for_unminted_passport() {
	new_test_ext().execute_with(|| {
		create_community();
		mint_passport();

		let avatar: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_avatar: BoundedVec<u8, ConstU32<60>> = avatar.try_into().unwrap();

		assert_noop!(
			Passport::update_avatar(RuntimeOrigin::signed(2), 1, bounded_avatar),
			Error::<Test>::PassportNotAvailable
		);
	});
}
