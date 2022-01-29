use std::marker::PhantomData;

use frame_support::dispatch::DispatchInfo;
use frame_support::traits::OnInitialize;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::SignedExtension;
use sp_runtime::transaction_validity::InvalidTransaction;

use crate::{mock::*, CountedMap, WatchDummy};

#[test]
fn it_works_for_optional_value() {
	new_test_ext().execute_with(|| {
		// Check that GenesisBuilder works properly.
		let val1 = 42;
		let val2 = 27;
		assert_eq!(Example::dummy(), Some(val1));

		// Check that accumulate works when we have Some value in Dummy already.
		assert_ok!(Example::accumulate_dummy(Origin::signed(1), val2));
		assert_eq!(Example::dummy(), Some(val1 + val2));

		// Check that accumulate works when we Dummy has None in it.
		<Example as OnInitialize<u64>>::on_initialize(2);
		assert_ok!(Example::accumulate_dummy(Origin::signed(1), val1));
		assert_eq!(Example::dummy(), Some(val1 + val2 + val1));
	});
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert_eq!(Example::foo(), 24);
		assert_ok!(Example::accumulate_foo(Origin::signed(1), 1));
		assert_eq!(Example::foo(), 25);
	});
}

#[test]
fn set_dummy_works() {
	new_test_ext().execute_with(|| {
		let test_val = 133;
		assert_ok!(Example::set_dummy(Origin::root(), test_val.into()));
		assert_eq!(Example::dummy(), Some(test_val));
	});
}

#[test]
fn signed_ext_watch_dummy_works() {
	new_test_ext().execute_with(|| {
		let call = Example::Call::set_dummy { new_value: 10 }.into();
		let info = DispatchInfo::default();

		assert_eq!(
			WatchDummy::<Test>(PhantomData)
				.validate(&1, &call, &info, 150)
				.unwrap()
				.priority,
			u64::MAX,
		);
		assert_eq!(
			WatchDummy::<Test>(PhantomData).validate(&1, &call, &info, 250),
			InvalidTransaction::ExhaustsResources.into(),
		);
	})
}

#[test]
fn counted_map_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(CountedMap::<Test>::count(), 0);
		CountedMap::<Test>::insert(3, 3);
		assert_eq!(CountedMap::<Test>::count(), 1);
	})
}

#[test]
fn weights_work() {
	// must have a defined weight.
	let default_call = Example::Call::<Test>::accumulate_dummy { increase_by: 10 };
	let info1 = default_call.get_dispatch_info();
	// aka. `let info = <Call<Test> as GetDispatchInfo>::get_dispatch_info(&default_call);`
	assert!(info1.weight > 0);

	// `set_dummy` is simpler than `accumulate_dummy`, and the weight
	//   should be less.
	let custom_call = Example::Call::<Test>::set_dummy { new_value: 20 };
	let info2 = custom_call.get_dispatch_info();
	assert!(info1.weight > info2.weight);
}
