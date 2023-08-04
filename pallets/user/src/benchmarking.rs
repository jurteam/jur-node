//! Benchmarking setup for pallet-user
//!
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as User;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	update_user {
		let caller: T::AccountId = whitelisted_caller();
	}: _(
		RawOrigin::Signed(caller.clone()),
		Some("Alice".as_bytes().to_vec().try_into().unwrap()),
		Some("Avatar".as_bytes().to_vec().try_into().unwrap())
	)
	verify {
		assert_last_event::<T>(Event::<T>::UserDetailsUpdated(caller).into());
	}

	impl_benchmark_test_suite!(User, crate::mock::new_test_ext(), crate::mock::Test);
}
