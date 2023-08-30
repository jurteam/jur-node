use crate::Users;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn update_user_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(User::update_user(
			RuntimeOrigin::signed(1),
			Some("Alice".as_bytes().to_vec().try_into().unwrap()),
			Some("avatar".as_bytes().to_vec().try_into().unwrap())
		));

		assert!(Users::<Test>::contains_key(1));
	});
}

#[test]
fn update_user_not_works_with_existing_name() {
	new_test_ext().execute_with(|| {
		assert_ok!(User::update_user(
			RuntimeOrigin::signed(1),
			Some("Alice".as_bytes().to_vec().try_into().unwrap()),
			Some("avatar".as_bytes().to_vec().try_into().unwrap())
		));
		assert_noop!(
			User::update_user(
				RuntimeOrigin::signed(2),
				Some("Alice".as_bytes().to_vec().try_into().unwrap()),
				Some("avatar".as_bytes().to_vec().try_into().unwrap())
			),
			Error::<Test>::UsernameNotAvailable
		);
	});
}
