use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(MintTokenModule::mint(Origin::signed(1), 12));
	});
}
