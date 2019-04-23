use std::fs::File;
use std::io::Read;

use futures::future::Future;
use pretty_assertions::assert_eq;

use rutebot::requests::forward_message::ForwardMessage;
use rutebot::requests::get_file::GetFile;
use rutebot::requests::get_me::GetMe;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::send_chat_action::{ChatAction, SendChatAction};
use rutebot::requests::send_message::{FileKind, InlineKeyboard, InlineKeyboardButton, ParseMode, ReplyMarkup};
use rutebot::requests::send_message::send_audio::SendAudio;
use rutebot::requests::send_message::send_document::SendDocument;
use rutebot::requests::send_message::send_photo::SendPhoto;
use rutebot::requests::send_message::send_text::SendText;
use rutebot::responses::{Audio, Document, Message, MessageEntityValue, PhotoSize, Update, User};

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

    let response: Document = run_one(rutebot.prepare_api_request(request).send()).document.unwrap();

    let downloaded_document = run_one(rutebot.prepare_api_request(GetFile::new(&response.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(response.file_size, Some(5));
    assert_eq!(response.file_name, Some("superfile".to_owned()));
    assert_eq!(downloaded_document, vec![1, 2, 3, 4, 5]);
}

#[test]
pub fn send_photo_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg").unwrap().read_to_end(&mut photo_content).unwrap();
    let photo_size = photo_content.len();
    let request =
        SendPhoto::new(chat_id,
                       FileKind::InputFile {
                           name: "superphoto",
                           content: photo_content,
                       });

    let response: Vec<PhotoSize> = run_one(rutebot.prepare_api_request(request).send()).photo.unwrap();
    let last_photo = response.last().unwrap();

    let downloaded_photo = run_one(rutebot.prepare_api_request(GetFile::new(&last_photo.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(downloaded_photo.len(), photo_size);
}

#[test]
pub fn send_audio_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut audio_content = Vec::new();
    File::open("./tests/Dark_Tranquility.mp3").unwrap().read_to_end(&mut audio_content).unwrap();
    let audio_size = audio_content.len();
    let request =
        SendAudio {
            performer: Some("Dark_Tranquility"),
            ..SendAudio::new(chat_id,
                             FileKind::InputFile {
                                 name: "superaudio",
                                 content: audio_content,
                             })
        };

    let response: Audio = run_one(rutebot.prepare_api_request(request).send()).audio.unwrap();

    let downloaded_audio = run_one(rutebot.prepare_api_request(GetFile::new(&response.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(downloaded_audio.len(), audio_size);
}

#[test]
pub fn forward_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let sent_msg: Message = run_one(rutebot.prepare_api_request(SendText::new(chat_id, "test")).send());

    let response: Message = run_one(rutebot.prepare_api_request(ForwardMessage::new(chat_id, chat_id, sent_msg.message_id)).send());

    assert_eq!(sent_msg.text, response.text);
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
    let entities = response.entities.unwrap();
    let values = entities.iter().filter_map(|x| x.extract_value(text)).collect::<Vec<MessageEntityValue>>();
    let message_entity = values.first();

    match message_entity {
        Some(MessageEntityValue::TextLink { link, text }) => {
            assert_eq!(link.as_str(), "http://example.com/");
            assert_eq!(text.as_str(), "экзамл.ком");
        }
        x => panic!("wrong message entity: {:?}", x)
    }
}