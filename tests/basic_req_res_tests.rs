use futures::future::Future;
use pretty_assertions::assert_eq;

use rutebot::requests::get_file::GetFile;
use rutebot::requests::get_me::GetMe;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::send_chat_action::{ChatAction, SendChatAction};
use rutebot::requests::send_message::{FileKind, InlineKeyboard, InlineKeyboardButton, ParseMode, ReplyMarkup};
use rutebot::requests::send_message::send_document::SendDocument;
use rutebot::requests::send_message::send_text::SendText;
use rutebot::responses::{Document, Message, MessageEntityValue, Update, User};

use crate::common::run_one;

mod common;

#[test]
pub fn get_me_works() {
    let rutebot = common::create_client();

    let response: User = run_one(rutebot.prepare_api_request(GetMe).send());

    assert_eq!(response.is_bot, true);
}

#[test]
pub fn send_text_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: Message = run_one(rutebot.prepare_api_request(SendText::new(chat_id, "Some text")).send());

    assert_eq!(response.text.unwrap(), "Some text");
}

#[test]
pub fn send_chat_action_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: bool = run_one(rutebot.prepare_api_request(SendChatAction::new(chat_id, ChatAction::Typing)).send());

    assert_eq!(response, true);
}

#[test]
pub fn get_updates_works() {
    let rutebot = common::create_client();

    let _response: Vec<Update> = run_one(rutebot.prepare_api_request(GetUpdates::new()).send());
}

#[test]
pub fn send_document_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let keyboard = InlineKeyboard {
        inline_keyboard: &[vec![InlineKeyboardButton::CallbackData {
            text: "text",
            callback_data: "data",
        }]]
    };
    let request =
        SendDocument {
            caption: Some("random file"),
            reply_markup: Some(ReplyMarkup::InlineKeyboard(keyboard)),
            ..SendDocument::new(chat_id,
                                FileKind::InputFile {
                                    name: "superfile",
                                    content: vec![1, 2, 3, 4, 5],
                                })
        };

    let response_document: Document = run_one(rutebot.prepare_api_request(request).send()).document.unwrap();
    let downloaded = run_one(rutebot.prepare_api_request(GetFile::new(&response_document.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));

    assert_eq!(response_document.file_size, Some(5));
    assert_eq!(response_document.file_name, Some("superfile".to_owned()));
    assert_eq!(downloaded, vec![1, 2, 3, 4, 5]);
}

#[test]
pub fn message_entity_values_extracted_correctly() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let weird_text = "великий и могучий: [экзамл.ком](http://example.com/) очень могучий и великий";

    let response: Message = run_one(rutebot.prepare_api_request(
        SendText {
            parse_mode: Some(ParseMode::Markdown),
            ..SendText::new(chat_id, weird_text)
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