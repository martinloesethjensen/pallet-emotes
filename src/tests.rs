use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_err, assert_noop};
use crate::Error::NoneValue;

#[test]
fn first_example_test() {
    new_test_ext().execute_with(|| {
        assert_err!(
            Emotes::emote(Origin::signed(1), String::from("bsp"),String::from("like")),Error::<Test>::NoneValue
        );
    })
}


