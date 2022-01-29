use crate::{mock::*};
use frame_support::{assert_noop, assert_ok};


#[test]
fn it_works_for_linear() {
	new_test_ext().execute_with(|| {
		assert_ok!(WeightsModule::store_value(Origin::signed(1), 42));
		assert_ok!(WeightsModule::add_n(Origin::signed(1), 420));
		assert_ok!(WeightsModule::double(Origin::signed(1), 462));
	});
}


#[test]
fn it_works_for_quadratic() {
	new_test_ext().execute_with(|| {
		assert_ok!(WeightsModule::store_value(Origin::signed(1), 42));
		assert_ok!(WeightsModule::complex_calculations(Origin::signed(1), 420, 420));
	});
}


#[test]
fn it_works_for_conditional() {
	new_test_ext().execute_with(|| {
		assert_ok!(WeightsModule::store_value(Origin::signed(1), 42));
		assert_ok!(WeightsModule::add_or_set(Origin::signed(1), true, 420));
	});
}
