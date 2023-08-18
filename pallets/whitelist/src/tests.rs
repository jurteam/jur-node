use crate::mock::Whitelist;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::error::BadOrigin;

#[test]
fn add_founder_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_founder(RuntimeOrigin::root(), 1));
	});
}

#[test]
fn add_founder_works_failed_with_founder_exist() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_founder(RuntimeOrigin::root(), 1));
		assert_noop!(
			Whitelist::add_founder(RuntimeOrigin::root(), 1),
			Error::<Test>::AlreadyFounder
		);
	});
}

#[test]
fn add_founder_works_failed_with_non_sudo_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::add_founder(RuntimeOrigin::signed(1), 1),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn revoke_founder_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_founder(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::revoke_founder(RuntimeOrigin::root(), 1));
	});
}

#[test]
fn revoke_founder_works_failed_with_founder_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::revoke_founder(RuntimeOrigin::root(), 1),
			Error::<Test>::FounderNotExist
		);
	});
}

#[test]
fn revoke_founder_works_failed_with_non_sudo_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::revoke_founder(RuntimeOrigin::signed(1), 1),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn add_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::root(), 1));
	});
}

#[test]
fn add_admin_works_failed_with_admin_exist() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::root(), 1));
		assert_noop!(Whitelist::add_admin(RuntimeOrigin::root(), 1), Error::<Test>::AlreadyAdmin);
	});
}

#[test]
fn add_admin_works_failed_with_non_sudo_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::add_admin(RuntimeOrigin::signed(1), 1),
			BadOrigin
		);
	});
}

#[test]
fn revoke_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::revoke_admin(RuntimeOrigin::root(), 1));
	});
}

#[test]
fn revoke_admin_works_failed_with_admin_not_exist() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::revoke_admin(RuntimeOrigin::root(), 1),
			Error::<Test>::AdminNotExist
		);
	});
}

#[test]
fn revoke_admin_works_failed_with_non_sudo_user() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Whitelist::revoke_admin(RuntimeOrigin::signed(1), 1),
			BadOrigin
		);
	});
}

#[test]
fn add_founder_with_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::add_founder(RuntimeOrigin::signed(1), 2));
	});
}

#[test]
fn revoke_founder_works_with_admin() {
	new_test_ext().execute_with(|| {
		assert_ok!(Whitelist::add_admin(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::add_founder(RuntimeOrigin::root(), 1));
		assert_ok!(Whitelist::revoke_founder(RuntimeOrigin::root(), 1));
	});
}
