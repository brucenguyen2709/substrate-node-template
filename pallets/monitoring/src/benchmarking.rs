//! Benchmarking setup for pallet-monitoring

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		/* set-up initial state */
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)/* the code to be benchmarked */
	verify {/* verifying final state */
		assert_eq!(Something::<T>::get(), Some(s));
	}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
