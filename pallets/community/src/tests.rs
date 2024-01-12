use crate::types::Category;
use crate::{
	mock::*,
	types::{CommunityMetaData, CommunityType},
	Communities, Customs, Error, Languages, Religions, Traditions, Values,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));
		setup_blocks(5);
		Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur1".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
					.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		)
			.unwrap();
		assert_ne!(
			Some(Communities::<Test>::get(2).unwrap().reference_id),
			Some(Communities::<Test>::get(1).unwrap().reference_id)
		);
	});
}

#[test]
fn create_community_should_work_when_founder_has_morethan_required_balance() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(99000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));
		setup_blocks(5);
	});
}

#[test]
fn create_community_should_not_work_if_founder_balance_is_below_required_balance() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(1000);
		set_required_balance_to_create_community(10000000000000000000);
		assert_noop!(
			Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"    JUR       ".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
					.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		),
			Error::<Test>::InsufficientBalanceToBecomeFounder
		);

		assert!(!Communities::<Test>::contains_key(1));

		setup_blocks(5);
	});
}

#[test]
fn create_community_not_works_with_duplicate_name() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));
		setup_blocks(5);
		assert_noop!(
			Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"    JUR       ".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
					.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		),
			Error::<Test>::CommunityAlreadyExist
		);
	});
}

#[test]
fn founder_with_more_communities_not_allowed() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur1".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
					.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		)
			.unwrap();
		Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur2".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
					.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		)
			.unwrap();
		assert_noop!(
			Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
			"Jur3".into(),
			Some(
				"Jur is the core community of the Jur ecosystem, which includes all the contributors."
				.into(),
			),
			Some(vec![1, 2]),
			Some(get_metadata()),
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation)
		),
			Error::<Test>::TooManyCommunities
		);
	});
}

#[test]
fn create_community_works_only_with_name() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		Community::create_community(
			RuntimeOrigin::signed(1),
			// hash of IPFS path of dummy logo
			None,
			"Jur".into(),
			None,
			None,
			None,
			Category::Public,
			Some("tag".into()),
			Some("#222307".into()),
			Some("#E76080".into()),
			Some(CommunityType::Nation),
		)
		.unwrap();
		assert!(Communities::<Test>::contains_key(1));
		assert_eq!(Communities::<Test>::get(1).unwrap().name.to_vec(), "Jur".as_bytes().to_vec());
	});
}

#[test]
fn create_community_not_works_with_invalid_color() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		assert_noop!(
			Community::create_community(
				RuntimeOrigin::signed(1),
				// hash of IPFS path of dummy logo
				None,
				"Jur".into(),
				None,
				None,
				None,
				Category::Public,
				Some("tag".into()),
				Some("#invalid color".into()),
				Some("#E76080".into()),
				Some(CommunityType::Nation)
			),
			Error::<Test>::BadColor
		);

		assert_noop!(
			Community::create_community(
				RuntimeOrigin::signed(1),
				// hash of IPFS path of dummy logo
				None,
				"Jur".into(),
				None,
				None,
				None,
				Category::Public,
				Some("tag".into()),
				Some("#E76080".into()),
				Some("#invalid color".into()),
				Some(CommunityType::Nation)
			),
			Error::<Test>::BadColor
		);
	});
}

#[test]
fn update_community_not_works_for_invalid_input() {
	let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
	let description = "Jur is the core community of the Jur ecosystem";

	new_test_ext().execute_with(|| {
		assert_noop!(
			Community::update_community(
				RuntimeOrigin::signed(1),
				0,
				Some(logo.into()),
				Some(description.into())
			),
			Error::<Test>::CommunityNotExist
		);
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_noop!(
			Community::update_community(
				RuntimeOrigin::signed(2),
				1,
				Some(logo.into()),
				Some(description.into())
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn update_community_works() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_eq!(
			Communities::<Test>::get(1).unwrap().logo.to_vec(),
			"bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq"
				.as_bytes()
				.to_vec()
		);

		let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
		let description = "Jur is the core community of the Jur ecosystem";

		assert_ok!(Community::update_community(
			RuntimeOrigin::signed(1),
			1,
			Some(logo.into()),
			Some(description.into())
		));

		assert_eq!(Communities::<Test>::get(1).unwrap().logo.to_vec(), logo.as_bytes().to_vec());
	});
}

#[test]
fn accept_members_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		let new_members = vec![3, 4];
		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_ok!(Community::accept_members(RuntimeOrigin::signed(1), 1, new_members));
		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2, 3, 4]);
	});
}

#[test]
fn accept_members_not_works_for_invalid_input() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));

		let new_members = vec![3, 4];

		assert_noop!(
			Community::accept_members(RuntimeOrigin::signed(1), 1, new_members.clone()),
			Error::<Test>::CommunityNotExist
		);
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::accept_members(RuntimeOrigin::signed(2), 1, new_members),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn update_metadata_works() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_eq!(
			Communities::<Test>::get(1)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec![
				Languages("English".as_bytes().to_vec().try_into().unwrap()),
				Languages("German".as_bytes().to_vec().try_into().unwrap())
			])
		);

		let custom_one: Vec<u8> =
			"in public transport young people should leave the seat to elderly or pregnant women"
				.into();
		let custom_two: Vec<u8> = "name newborns with a name that starts with the letter A".into();

		let languages_1: Vec<u8> = "Spanish".into();
		let languages_2: Vec<u8> = "Swish".into();

		let religions_1: Vec<u8> = "Christianity".into();
		let religions_2: Vec<u8> = "Buddhism".into();

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
			territories: Some(vec![]),
			traditions: Some(vec![
				Traditions(traditions_1.try_into().unwrap()),
				Traditions(traditions_2.try_into().unwrap()),
			]),
			values: Some(vec![
				Values(values_1.try_into().unwrap()),
				Values(values_2.try_into().unwrap()),
			]),
		};

		assert_ok!(Community::update_metadata(RuntimeOrigin::signed(1), 1, community_metadata));

		assert_eq!(
			Communities::<Test>::get(1)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec![
				Languages("Spanish".as_bytes().to_vec().try_into().unwrap()),
				Languages("Swish".as_bytes().to_vec().try_into().unwrap())
			])
		);
	});
}

#[test]
fn update_metadata_not_works_for_invalid_community_id() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_eq!(
			Communities::<Test>::get(1)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec![
				Languages("English".as_bytes().to_vec().try_into().unwrap()),
				Languages("German".as_bytes().to_vec().try_into().unwrap())
			])
		);

		assert_noop!(
			Community::update_metadata(RuntimeOrigin::signed(1), 2, get_metadata()),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn update_metadata_not_works_for_invalid_caller() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_eq!(
			Communities::<Test>::get(1)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec![
				Languages("English".as_bytes().to_vec().try_into().unwrap()),
				Languages("German".as_bytes().to_vec().try_into().unwrap())
			])
		);

		assert_noop!(
			Community::update_metadata(RuntimeOrigin::signed(2), 1, get_metadata()),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn join_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_ok!(Community::join_community(RuntimeOrigin::signed(3), 1));
		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2, 3]);
	});
}

#[test]
fn join_community_not_works_for_already_joined() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);
		assert_noop!(
			Community::join_community(RuntimeOrigin::signed(2), 1),
			Error::<Test>::AlreadyMember
		);
	});
}

#[test]
fn join_community_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);
		assert_noop!(
			Community::join_community(RuntimeOrigin::signed(2), 2),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn leave_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_ok!(Community::leave_community(RuntimeOrigin::signed(2), 1));
		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1]);
	});
}

#[test]
fn leave_community_not_work_for_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::leave_community(RuntimeOrigin::signed(3), 1),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn leave_community_not_work_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::leave_community(RuntimeOrigin::signed(2), 2),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn remove_member_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_ok!(Community::remove_member(RuntimeOrigin::signed(1), 2, 1));
		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1]);
	});
}

#[test]
fn remove_member_not_work_for_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::remove_member(RuntimeOrigin::signed(1), 3, 1),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn remove_member_not_work_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(1));
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();

		assert_eq!(Communities::<Test>::get(1).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::remove_member(RuntimeOrigin::signed(1), 2, 2),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn update_community_tag_and_colors_works() {
	new_test_ext().execute_with(|| {
		set_balance(10000000000000000000);
		set_required_balance_to_create_community(10000000000000000000);
		create_community();
		assert!(Communities::<Test>::contains_key(1));

		assert_eq!(Communities::<Test>::get(1).unwrap().tag, "tag".as_bytes().to_vec());

		assert_eq!(
			Communities::<Test>::get(1).unwrap().primary_color,
			"#222307".as_bytes().to_vec()
		);

		assert_eq!(
			Communities::<Test>::get(1).unwrap().secondary_color,
			"#E76080".as_bytes().to_vec()
		);

		let tag = "Alpha";
		let p_color = "#E76081";
		let s_color = "#222308";

		assert_ok!(Community::update_passport_metadata(
			RuntimeOrigin::signed(1),
			1,
			Some(tag.into()),
			Some(p_color.into()),
			Some(s_color.into())
		));

		assert_eq!(Communities::<Test>::get(1).unwrap().tag, tag.as_bytes().to_vec());
		assert_eq!(Communities::<Test>::get(1).unwrap().primary_color, p_color.as_bytes().to_vec());
		assert_eq!(
			Communities::<Test>::get(1).unwrap().secondary_color,
			s_color.as_bytes().to_vec()
		);
	});
}
