use crate::{mock::*, Choices, Error, Votes};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};
use pallet_community::types::{CommunityMetaData, CommunityType};
use sp_core::H256;

fn get_community_metadata() -> CommunityMetaData<u64, H256> {
	let community_metadata = CommunityMetaData {
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
	};

	community_metadata
}
fn create_community() {
	Community::create_community(
		RuntimeOrigin::signed(1),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		"Jur is the core community of the Jur ecosystem, which includes all the contributors."
			.into(),
		vec![1, 2],
		Some(get_community_metadata()),
	)
	.unwrap();
}

fn create_proposal() {
	let proposal_address: Vec<u8> =
		"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
	let bounded_proposal_address: BoundedVec<u8, ConstU32<60>> =
		proposal_address.try_into().unwrap();
	create_community();
	Proposal::create_proposal(
		RuntimeOrigin::signed(1),
		0,
		bounded_proposal_address,
		"Which language should we speak within the Community?".into(),
		vec![
			"English".as_bytes().to_vec(),
			"Ghukliak".as_bytes().to_vec(),
			"官话".as_bytes().to_vec(),
			"Rust".as_bytes().to_vec(),
		],
		false,
		5
	)
	.unwrap();
}

#[test]
fn create_proposal_works() {
	new_test_ext().execute_with(|| {
		let proposal_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_proposal_address: BoundedVec<u8, ConstU32<60>> =
			proposal_address.try_into().unwrap();

		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			0,
			bounded_proposal_address,
			"Which is your native country".into(),
			vec![
				"India".as_bytes().to_vec(),
				"Germany".as_bytes().to_vec(),
				"England".as_bytes().to_vec()
			],
			false,
			5
		));

		assert!(Choices::<Test>::contains_key(0));
	});
}

#[test]
fn create_proposal_does_not_work_when_no_community_id() {
	new_test_ext().execute_with(|| {
		let proposal_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_proposal_address: BoundedVec<u8, ConstU32<60>> =
			proposal_address.try_into().unwrap();

		assert_noop!(
			Proposal::create_proposal(
				RuntimeOrigin::signed(1),
				0,
				bounded_proposal_address,
				"Which is your native country".into(),
				vec![
					"India".as_bytes().to_vec(),
					"Germany".as_bytes().to_vec(),
					"England".as_bytes().to_vec()
				],
				false,
				5
			),
			Error::<Test>::CommunityDoesNotExist
		);
	});
}

#[test]
fn submit_choice_works() {
	new_test_ext().execute_with(|| {
		create_proposal();
		assert_ok!(Proposal::submit_choice(RuntimeOrigin::signed(1), 0, 0, 1,));

		assert_eq!(Votes::<Test>::get(1).vote_count, 1);
	});
}

#[test]
fn submit_proposal_not_work_for_invalid_input() {
	new_test_ext().execute_with(|| {
		create_proposal();
		assert_noop!(
			Proposal::submit_choice(RuntimeOrigin::signed(1), 0, 11, 2,),
			Error::<Test>::ProposalDoesNotExist
		);
	});
}

#[test]
fn submit_choices_not_work_for_invalid_input() {
	new_test_ext().execute_with(|| {
		let proposal_address: Vec<u8> =
			"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_proposal_address: BoundedVec<u8, ConstU32<60>> =
			proposal_address.try_into().unwrap();

		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			0,
			bounded_proposal_address.clone(),
			"Which is your native language".into(),
			vec![],
			false,
			5
		));

		assert_noop!(
			Proposal::submit_choice(RuntimeOrigin::signed(1), 0, 0, 1,),
			Error::<Test>::NoChoiceAvailable
		);

		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			0,
			bounded_proposal_address,
			"Which Language".into(),
			vec!["English".as_bytes().to_vec()],
			false,
			5
		));

		assert_noop!(
			Proposal::submit_choice(RuntimeOrigin::signed(1), 0, 1, 3,),
			Error::<Test>::ChoiceDoesNotExist
		);
	});
}