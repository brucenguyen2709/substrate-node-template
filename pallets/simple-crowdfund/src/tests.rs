use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(SimpleCrowdfundModule::create(Origin::signed(1), 444, 10, 1));
	});
}
