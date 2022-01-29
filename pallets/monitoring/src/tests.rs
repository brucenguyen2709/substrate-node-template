use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(MonitoringModule::create_claim(Origin::signed(1), vec![1, 2, 3]));

		// Read pallet storage and assert an expected result.
		//assert_eq!(MonitoringModule::something(), Some(vec![1, 2, 3]));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		//assert_noop!(MonitoringModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
