use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_lock_capital() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(LockableCurrencyModule::lock_capital(Origin::signed(1), 42));
	});
}
