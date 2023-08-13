//! Benchmarking setup for pallet-whitelist
//!
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Whitelist;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	add_founder {
		let caller: T::AccountId = whitelisted_caller();
	}: _(
		RawOrigin::Root,
		caller.clone()
	)
	verify {
		assert_last_event::<T>(Event::<T>::AddedFounder(caller).into());
	}

	revoke_founder {
		let caller: T::AccountId = whitelisted_caller();
		Whitelist::<T>::add_founder(
			RawOrigin::Root.into(),
			caller.clone()
		).unwrap();

	}: _(
		RawOrigin::Root,
		caller.clone()
	)
	verify {
		assert_last_event::<T>(Event::<T>::RevokedFounder(caller).into());
	}

	add_admin {
		let caller: T::AccountId = whitelisted_caller();
	}: _(
		RawOrigin::Root,
		caller.clone()
	)
	verify {
		assert_last_event::<T>(Event::<T>::AddedAdmin(caller).into());
	}

	revoke_admin {
		let caller: T::AccountId = whitelisted_caller();
		Whitelist::<T>::add_admin(
			RawOrigin::Root.into(),
			caller.clone()
		).unwrap();

	}: _(
		RawOrigin::Root,
		caller.clone()
	)
	verify {
		assert_last_event::<T>(Event::<T>::RevokedAdmin(caller).into());
	}

	impl_benchmark_test_suite!(Whitelist, crate::mock::new_test_ext(), crate::mock::Test);
}
