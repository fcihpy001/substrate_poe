use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(PoeModule::create_claim(Origin::signed(1),vec![0x01]));
        assert_eq!(PoeModule::Proofs::get(), Some(0x03))
    })
}