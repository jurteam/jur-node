use crate::{
	mock::*,
	types::{CommunityMetaData, CommunityType},
	Communities, Error,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

fn get_metadata() -> CommunityMetaData<u64, H256> {
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
		Some(get_metadata()),
	)
	.unwrap();
}
#[test]
fn create_community_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();
		assert!(Communities::<Test>::contains_key(0));
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
		)
			.unwrap();
		assert!(Communities::<Test>::contains_key(0));
		assert_eq!(Communities::<Test>::get(0).unwrap().name.to_vec(), "Jur".as_bytes().to_vec());
	});
}

#[test]
fn delete_community_works() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_ok!(Community::delete_community(RuntimeOrigin::signed(1), 0));
		assert!(!Communities::<Test>::contains_key(0));
	});
}

#[test]
fn delete_community_not_works_for_invalid_input() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Community::delete_community(RuntimeOrigin::signed(1), 0),
			Error::<Test>::CommunityNotExist
		);

		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_noop!(
			Community::delete_community(RuntimeOrigin::signed(2), 0),
			Error::<Test>::NoPermission
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

        assert_eq!(Communities::<Test>::get(0).unwrap().logo.unwrap(), "bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".as_bytes().to_vec());

        let logo = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq";
        let description = "Jur is the core community of the Jur ecosystem";

        assert_ok!(Community::update_community(RuntimeOrigin::signed(1), 0, Some(logo.into()), Some(description.into())));

        assert_eq!(Communities::<Test>::get(0).unwrap().logo.unwrap(), logo.as_bytes().to_vec());

    });
}

#[test]
fn add_members_works() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));
		create_community();

		let new_members = vec![3, 4];
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_ok!(Community::add_members(RuntimeOrigin::signed(1), 0, new_members));
		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2, 3, 4]);
	});
}

#[test]
fn add_members_not_works_for_invalid_input() {
	new_test_ext().execute_with(|| {
		assert!(!Communities::<Test>::contains_key(0));

		let new_members = vec![3, 4];

		assert_noop!(
			Community::add_members(RuntimeOrigin::signed(1), 1, new_members.clone()),
			Error::<Test>::CommunityNotExist
		);

		create_community();

		assert_eq!(Communities::<Test>::get(0).unwrap().members, vec![1, 2]);

		assert_noop!(
			Community::add_members(RuntimeOrigin::signed(2), 0, new_members),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn update_metadata_works() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().languages, Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()]));

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

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().languages, Some(vec!["Spanish".as_bytes().to_vec(), "Swish".as_bytes().to_vec()]));

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().territories, None);

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().norms, None);

	});
}

#[test]
fn update_metadata_not_works_for_invalid_community_id() {
	new_test_ext().execute_with(|| {
		create_community();
		assert!(Communities::<Test>::contains_key(0));

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().languages, Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()]));

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

		assert_eq!(Communities::<Test>::get(0).unwrap().metadata.unwrap().languages, Some(vec!["English".as_bytes().to_vec(), "German".as_bytes().to_vec()]));

		assert_noop!(
			Community::update_metadata(RuntimeOrigin::signed(2), 0, get_metadata()),
			Error::<Test>::NoPermission
		);
	});
}
