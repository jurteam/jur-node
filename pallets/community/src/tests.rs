use crate::{
	mock::*,
	types::{CommunityMetaData, CommunityType},
	Communities, Error,
};
use frame_support::{assert_noop, assert_ok};
use crate::types::Category;

#[test]
fn create_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();
		assert!(Communities::<Test>::contains_key(0));
		setup_blocks(5);
		create_community();
		assert_ne!(
			Some(Communities::<Test>::get(1).unwrap().reference_id),
			Some(Communities::<Test>::get(0).unwrap().reference_id)
		);
	});
}

#[test]
fn create_community_works_only_with_name() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
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
			Some("#E76080".into())
		)
		.unwrap();
		assert!(Communities::<Test>::contains_key(0));
		assert_eq!(Communities::<Test>::get(0).unwrap().name.to_vec(), "Jur".as_bytes().to_vec());
	});
}

#[test]
fn create_community_not_works_with_invalid_color() {
	new_test_ext().execute_with(|| {
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
			Some("#E76080".into())
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
			Some("#invalid color".into())
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

		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_noop!(
			Community::update_community(
				RuntimeOrigin::signed(2),
				0,
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
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(
			Communities::<Test>::get(0).unwrap().logo.unwrap(),
			"bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq"
				.as_bytes()
				.to_vec()
		);

		let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
		let description = "Jur is the core community of the Jur ecosystem";

		assert_ok!(Community::update_community(
			RuntimeOrigin::signed(1),
			0,
			Some(logo.into()),
			Some(description.into())
		));

		assert_eq!(Communities::<Test>::get(0).unwrap().logo.unwrap(), logo.as_bytes().to_vec());
	});
}

#[test]
fn accept_members_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
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
			Some(get_metadata()),
			Category::Public,
            Some("tag".into()),
            Some("#222307".into()),
            Some("#E76080".into())
		)
			.unwrap();

		let new_members = vec![3, 4];
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_ok!(Community::accept_members(RuntimeOrigin::signed(1), 0, new_members));
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2, 3, 4]);
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

		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::accept_members(RuntimeOrigin::signed(2), 0, new_members),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn update_metadata_works() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(
			Communities::<Test>::get(0)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()])
		);

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

		assert_ok!(Community::update_metadata(RuntimeOrigin::signed(1), 0, community_metadata));

		assert_eq!(
			Communities::<Test>::get(0)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec!["Spanish".as_bytes().to_vec(), "Swish".as_bytes().to_vec()])
		);

		assert_eq!(
			Communities::<Test>::get(0)
				.unwrap()
				.metadata
				.unwrap()
				.territories,
			None
		);

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().norms, None);
	});
}

#[test]
fn update_metadata_not_works_for_invalid_community_id() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(
			Communities::<Test>::get(0)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()])
		);

		assert_noop!(
			Community::update_metadata(RuntimeOrigin::signed(1), 1, get_metadata()),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn update_metadata_not_works_for_invalid_caller() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(
			Communities::<Test>::get(0)
				.unwrap()
				.metadata
				.unwrap()
				.languages,
			Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()])
		);

		assert_noop!(
			Community::update_metadata(RuntimeOrigin::signed(2), 0, get_metadata()),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn join_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_ok!(Community::join_community(RuntimeOrigin::signed(3), 0));
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2, 3]);
	});
}

#[test]
fn join_community_not_works_for_already_joined() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);
		assert_noop!(
			Community::join_community(RuntimeOrigin::signed(2), 0),
			Error::<Test>::AlreadyMember
		);
	});
}

#[test]
fn join_community_not_works_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);
		assert_noop!(
			Community::join_community(RuntimeOrigin::signed(2), 1),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn leave_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_ok!(Community::leave_community(RuntimeOrigin::signed(2), 0));
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1]);
	});
}

#[test]
fn leave_community_not_work_for_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::leave_community(RuntimeOrigin::signed(3), 0),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn leave_community_not_work_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::leave_community(RuntimeOrigin::signed(2), 1),
			Error::<Test>::CommunityNotExist
		);
	});
}

#[test]
fn remove_member_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_ok!(Community::remove_member(RuntimeOrigin::signed(1), 2, 0));
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1]);
	});
}

#[test]
fn remove_member_not_work_for_member_not_part_of_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::remove_member(RuntimeOrigin::signed(1), 3, 0),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn remove_member_not_work_for_invalid_community() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::remove_member(RuntimeOrigin::signed(1), 2, 1),
			Error::<Test>::CommunityNotExist
		);
	});
}
