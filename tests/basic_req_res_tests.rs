use pretty_assertions::assert_eq;

use rutebot::requests::get_me::GetMe;
use rutebot::requests::get_updates::GetUpdatesRequest;
use rutebot::requests::send_chat_action::{ChatAction, SendChatAction};
use rutebot::requests::send_message::send_text_message::{ParseMode, SendTextMessageRequest};
use rutebot::responses::{Message, MessageEntityValue, Update, User};

use crate::common::run_one;

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

#[test]
pub fn send_chat_action_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: bool = run_one(rutebot.prepare_api_request(&SendChatAction::new(chat_id, ChatAction::Typing)).send());

    assert_eq!(response, true);
}

#[test]
pub fn get_updates_works() {
    let rutebot = common::create_client();

    let _response: Vec<Update> = run_one(rutebot.prepare_api_request(&GetUpdatesRequest::new()).send());
}

#[test]
pub fn message_entity_values_extracted_correctly() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let weird_text = "великий и могучий: [экзамл.ком](http://example.com/) очень могучий и великий";

    let response: Message = run_one(rutebot.prepare_api_request(
        &SendTextMessageRequest {
            parse_mode: Some(ParseMode::Markdown),
            ..SendTextMessageRequest::new(chat_id, weird_text)
        }).send());
    let text = &response.text.unwrap();
    let values = response.entities.unwrap().into_iter().map(|x| x.extract_value(text).unwrap()).collect::<Vec<MessageEntityValue>>();
    let message_entity = values.first();

    match message_entity {
        Some(MessageEntityValue::TextLink { link, text }) => {
            assert_eq!(link, "http://example.com/");
            assert_eq!(text, "экзамл.ком");
        }
        x => panic!("wrong message entity: {:?}", x)
    }
}