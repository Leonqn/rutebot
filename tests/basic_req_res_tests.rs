use futures::future::Future;
use pretty_assertions::{assert_eq, assert_ne};

use rutebot::requests::get_me::GetMe;
use rutebot::responses::User;

use crate::common::run_one;

mod common;

#[test]
pub fn get_me_works() {
    let rutebot = common::create_client();

    let response: User = run_one(rutebot.prepare_api_request(&GetMe).send());

    assert_eq!(response.is_bot, true);
}