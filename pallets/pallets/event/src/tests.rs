use crate::types::EventType;
use crate::{mock::*, Error, Events};
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

fn add_admin() {
	Whitelist::add_admin(RuntimeOrigin::root(), 2).unwrap();
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
	let badge_name: Vec<u8> = "EVENT".into();
	let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

	let badge_description: Vec<u8> = "Event for the jur community members".into();
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
	Passport::mint(RuntimeOrigin::signed(3), 1).unwrap();
}

#[test]
fn create_events_works() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		EventPallet::create_event(
			RuntimeOrigin::signed(1),
			1,
			bounded_event_name,
			bounded_events_description,
			1703745212,
			1703831612,
			EventType::Virtual,
			None,
			bounded_badge_name,
		)
		.unwrap();

		assert!(Events::<Test>::get(1, 1).is_some());
	});
}

#[test]
fn create_events_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			EventPallet::create_event(
				RuntimeOrigin::signed(1),
				5,
				bounded_event_name,
				bounded_events_description,
				1703745212,
				1703831612,
				EventType::Virtual,
				None,
				bounded_badge_name,
			),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn create_events_not_works_for_invalid_timestamp() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			EventPallet::create_event(
				RuntimeOrigin::signed(1),
				1,
				bounded_event_name,
				bounded_events_description,
				1733745212,
				1703831612,
				EventType::Virtual,
				None,
				bounded_badge_name,
			),
			Error::<Test>::InvalidEventTime
		);
	});
}

#[test]
fn create_events_not_works_for_invalid_founder() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		assert_noop!(
			EventPallet::create_event(
				RuntimeOrigin::signed(3),
				1,
				bounded_event_name,
				bounded_events_description,
				1703745212,
				1703831612,
				EventType::Virtual,
				None,
				bounded_badge_name,
			),
			Error::<Test>::NotAllowed
		);
	});
}

#[test]
fn proof_of_presence_works() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();
		add_admin();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		EventPallet::create_event(
			RuntimeOrigin::signed(1),
			1,
			bounded_event_name,
			bounded_events_description,
			1703745212,
			1703831612,
			EventType::Virtual,
			None,
			bounded_badge_name,
		)
		.unwrap();

		assert!(Events::<Test>::get(1, 1).is_some());

		EventPallet::proof_of_presence(RuntimeOrigin::signed(2), 1, 1, 3).unwrap();
	});
}

#[test]
fn proof_of_presence_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();
		add_admin();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		EventPallet::create_event(
			RuntimeOrigin::signed(1),
			1,
			bounded_event_name,
			bounded_events_description,
			1703745212,
			1703831612,
			EventType::Virtual,
			None,
			bounded_badge_name,
		)
		.unwrap();

		assert_noop!(
			EventPallet::proof_of_presence(RuntimeOrigin::signed(2), 3, 1, 3,),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn proof_of_presence_not_works_for_invalid_event() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();
		add_admin();

		assert_noop!(
			EventPallet::proof_of_presence(RuntimeOrigin::signed(2), 1, 1, 3,),
			Error::<Test>::EventDoesNotExist
		);
	});
}

#[test]
fn proof_of_presence_not_works_for_non_passport_holder() {
	new_test_ext().execute_with(|| {
		mint_passport_with_badge();
		add_admin();

		let event_name: Vec<u8> = "DEV Force".into();
		let bounded_event_name: BoundedVec<u8, ConstU32<512>> = event_name.try_into().unwrap();

		let event_description: Vec<u8> = "Event for the jur community members".into();
		let bounded_events_description: BoundedVec<u8, ConstU32<8192>> =
			event_description.try_into().unwrap();

		let badge_name: Vec<u8> = "EVENT".into();
		let bounded_badge_name: BoundedVec<u8, ConstU32<20>> = badge_name.try_into().unwrap();

		EventPallet::create_event(
			RuntimeOrigin::signed(1),
			1,
			bounded_event_name,
			bounded_events_description,
			1703745212,
			1703831612,
			EventType::Virtual,
			None,
			bounded_badge_name,
		)
		.unwrap();

		assert_noop!(
			EventPallet::proof_of_presence(RuntimeOrigin::signed(2), 1, 1, 4,),
			Error::<Test>::PassportNotAvailable
		);
	});
}
