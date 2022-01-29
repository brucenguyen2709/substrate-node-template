use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ProofOfExistence::create_claim(Origin::signed(1), vec![1]));
	});
}
