use pretty_assertions::{assert_eq, assert_ne};

use rutebot::requests::get_me::GetMe;
use rutebot::responses::{User, Message};

use crate::common::run_one;
use rutebot::requests::send_message::send_text_message::SendTextMessageRequest;

mod common;

#[test]
pub fn get_me_works() {
    let rutebot = common::create_client();

    let response: User = run_one(rutebot.prepare_api_request(&GetMe).send());

    assert_eq!(response.is_bot, true);
}

#[test]
pub fn send_text_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: Message = run_one(rutebot.prepare_api_request(&SendTextMessageRequest::new(chat_id, "Some text")).send());

    assert_eq!(response.text.unwrap(), "Some text");
}