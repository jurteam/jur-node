use crate::pallet::ProposalResult;
use crate::types::ProposalResultStatus;
use crate::{mock::*, Choices, Error, Votes};
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
		Some(vec![1, 2, 7, 8]),
		Some(get_community_metadata()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
	)
	.unwrap();
}

pub fn add_founder() {
	Whitelist::add_founder(RuntimeOrigin::root(), 1).unwrap();
}

fn create_proposal() {
	let proposal_name: Vec<u8> = "Jur community Language proposal".into();
	let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

	let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
	let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
		proposal_description.try_into().unwrap();

	add_founder();
	create_community();
	Proposal::create_proposal(
		RuntimeOrigin::signed(1),
		1,
		bounded_proposal_name,
		bounded_proposal_description,
		vec!["Yes".as_bytes().to_vec(), "No".as_bytes().to_vec()],
		false,
		5,
	)
	.unwrap();
}

#[test]
fn create_proposal_works() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name,
			bounded_proposal_description,
			vec![
				"India".as_bytes().to_vec(),
				"Germany".as_bytes().to_vec(),
				"England".as_bytes().to_vec()
			],
			false,
			5
		));

		assert!(Choices::<Test>::contains_key(1));
	});
}

#[test]
fn create_proposal_does_not_work_when_no_community_id() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		assert_noop!(
			Proposal::create_proposal(
				RuntimeOrigin::signed(1),
				1,
				bounded_proposal_name,
				bounded_proposal_description,
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
fn cast_vote_works() {
	new_test_ext().execute_with(|| {
		create_proposal();
		let choice: Vec<u8> = "Yes".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice));

		assert_eq!(Votes::<Test>::get(1).unwrap().vote_count, 1);
	});
}

#[test]
fn cast_vote_not_work_for_invalid_input() {
	new_test_ext().execute_with(|| {
		create_proposal();
		let choice: Vec<u8> = "No".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();
		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 11, bounded_choice),
			Error::<Test>::ProposalDoesNotExist
		);
	});
}

#[test]
fn cast_votes_not_work_for_invalid_input() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name.clone(),
			bounded_proposal_description.clone(),
			vec!["Yes".into(), "No".into()],
			false,
			5
		));

		let choice: Vec<u8> = "no".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice),
			Error::<Test>::ChoiceDoesNotExist
		);

		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name,
			bounded_proposal_description,
			vec!["English".into(), "German".into()],
			false,
			5
		));

		let choice: Vec<u8> = "No".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 2, bounded_choice),
			Error::<Test>::ChoiceDoesNotExist
		);
	});
}

#[test]
fn cast_vote_not_work_for_after_proposal_deadline() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name,
			bounded_proposal_description,
			vec![
				"English".as_bytes().to_vec(),
				"Ghukliak".as_bytes().to_vec(),
				"官话".as_bytes().to_vec(),
				"Rust".as_bytes().to_vec(),
			],
			false,
			1,
		));

		run_to_block(15_000);
		let choice: Vec<u8> = "English".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice),
			Error::<Test>::ProposalNotActive
		);
	});
}

#[test]
fn cast_vote_not_works_for_duplicate_vote() {
	new_test_ext().execute_with(|| {
		create_proposal();
		let choice: Vec<u8> = "Yes".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice.clone()));

		assert_eq!(Votes::<Test>::get(1).unwrap().vote_count, 1);

		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice),
			Error::<Test>::DuplicateVote
		);
	});
}

#[test]
fn cast_vote_not_works_for_account_limit_exceeds() {
	new_test_ext().execute_with(|| {
		create_proposal();
		let choice: Vec<u8> = "Yes".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice.clone()));
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(2), 1, 1, bounded_choice.clone()));
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(7), 1, 1, bounded_choice.clone()));

		assert_eq!(Votes::<Test>::get(1).unwrap().vote_count, 3);

		assert_noop!(
			Proposal::cast_vote(RuntimeOrigin::signed(8), 1, 1, bounded_choice),
			Error::<Test>::AccountLimitReached
		);
	});
}

#[test]
fn create_proposal_not_working_invalid_choice() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_noop!(
			Proposal::create_proposal(
				RuntimeOrigin::signed(1),
				1,
				bounded_proposal_name,
				bounded_proposal_description,
				vec![],
				false,
				5
			),
			Error::<Test>::InvalidChoicesGiven
		);
	});
}

#[test]
fn cast_vote_works_with_proposal_result_accepted() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name,
			bounded_proposal_description,
			vec!["Yes".as_bytes().to_vec(), "No".as_bytes().to_vec(),],
			false,
			1,
		));

		let choice: Vec<u8> = "Yes".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice.clone()));
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(2), 1, 1, bounded_choice.clone()));

		run_to_block(15_000);

		assert_eq!(Votes::<Test>::get(1).unwrap().vote_count, 2);
		assert_eq!(ProposalResult::<Test>::get(1).unwrap().0, ProposalResultStatus::Accepted);
	});
}

#[test]
fn cast_vote_works_with_proposal_result_rejected() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<60>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<250>> =
			proposal_description.try_into().unwrap();

		add_founder();
		create_community();
		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			1,
			bounded_proposal_name,
			bounded_proposal_description,
			vec!["Yes".as_bytes().to_vec(), "No".as_bytes().to_vec(),],
			false,
			1,
		));

		let choice: Vec<u8> = "Yes".into();
		let bounded_choice: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		let choice: Vec<u8> = "No".into();
		let bounded_choice2: BoundedVec<u8, ConstU32<10>> = choice.try_into().unwrap();

		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(1), 1, 1, bounded_choice));
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(2), 1, 1, bounded_choice2.clone()));
		assert_ok!(Proposal::cast_vote(RuntimeOrigin::signed(7), 1, 1, bounded_choice2));

		run_to_block(15_000);

		assert_eq!(Votes::<Test>::get(2).unwrap().vote_count, 2);
		assert_eq!(ProposalResult::<Test>::get(1).unwrap().0, ProposalResultStatus::Rejected);
	});
}
