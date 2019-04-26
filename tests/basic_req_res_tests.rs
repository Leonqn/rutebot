use std::fs::File;
use std::io::Read;

use futures::future::Future;
use pretty_assertions::assert_eq;

use rutebot::requests::{FileKind, InlineKeyboard, InlineKeyboardButton, InputMediaPhoto, InputMediaVideo, ParseMode, ReplyMarkup};
use rutebot::requests::edit_live_location::EditLiveLocation;
use rutebot::requests::export_chat_invite_link::ExportChatInviteLink;
use rutebot::requests::forward_message::ForwardMessage;
use rutebot::requests::get_file::GetFile;
use rutebot::requests::get_me::GetMe;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::get_user_profile_photos::GetUserProfilePhotos;
use rutebot::requests::send_animation::SendAnimation;
use rutebot::requests::send_audio::SendAudio;
use rutebot::requests::send_chat_action::{ChatAction, SendChatAction};
use rutebot::requests::send_contact::SendContact;
use rutebot::requests::send_document::SendDocument;
use rutebot::requests::send_location::SendLocation;
use rutebot::requests::send_media_group::{InputMediaPhotoOrVideo, SendMediaGroup};
use rutebot::requests::send_photo::SendPhoto;
use rutebot::requests::send_poll::SendPoll;
use rutebot::requests::send_text::SendText;
use rutebot::requests::send_venue::SendVenue;
use rutebot::requests::send_video::SendVideo;
use rutebot::requests::send_video_note::SendVideoNote;
use rutebot::requests::send_voice::SendVoice;
use rutebot::requests::set_chat_photo::SetChatPhoto;
use rutebot::requests::stop_live_location::StopLiveLocation;
use rutebot::responses::{Audio, Contact, Document, EditedLiveLocation, Message, MessageEntityValue, Poll, Update, User, UserProfilePhotos, Venue, Video, VideoNote, Voice};

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
    let request =
        SendPhoto::new(chat_id,
                       FileKind::InputFile {
                           name: "superphoto",
                           content: photo_content,
                       });

    let response: Message = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.photo.is_some(), true);
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
pub fn send_video_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_content = Vec::new();
    File::open("./tests/sample_video.mp4").unwrap().read_to_end(&mut video_content).unwrap();
    let video_size = video_content.len();
    let request =
        SendVideo::new(chat_id,
                       FileKind::InputFile {
                           name: "supervideo",
                           content: video_content,
                       });

    let response: Video = run_one(rutebot.prepare_api_request(request).send()).video.unwrap();

    let downloaded_video = run_one(rutebot.prepare_api_request(GetFile::new(&response.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(downloaded_video.len(), video_size);
}

#[test]
pub fn send_animation_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut gif_content = Vec::new();
    File::open("./tests/sample_gif.gif").unwrap().read_to_end(&mut gif_content).unwrap();
    let request = SendAnimation {
        width: Some(808),
        height: Some(538),
        ..SendAnimation::new(chat_id,
                             FileKind::InputFile {
                                 name: "supergif",
                                 content: gif_content,
                             })
    };

    let response: Message = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.animation.is_some(), true);
}

#[test]
pub fn send_voice_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut voice_content = Vec::new();
    File::open("./tests/sample_voice.ogg").unwrap().read_to_end(&mut voice_content).unwrap();
    let voice_size = voice_content.len();
    let request =
        SendVoice::new(chat_id,
                       FileKind::InputFile {
                           name: "supervoice",
                           content: voice_content,
                       });

    let response: Voice = run_one(rutebot.prepare_api_request(request).send()).voice.unwrap();

    let downloaded_voice = run_one(rutebot.prepare_api_request(GetFile::new(&response.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(downloaded_voice.len(), voice_size);
}

#[test]
pub fn send_video_note_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_note_content = Vec::new();
    File::open("./tests/sample_video_note.mp4").unwrap().read_to_end(&mut video_note_content).unwrap();
    let video_note_size = video_note_content.len();
    let request =
        SendVideoNote::new(chat_id,
                           FileKind::InputFile {
                               name: "supervideonote",
                               content: video_note_content,
                           });

    let response: VideoNote = run_one(rutebot.prepare_api_request(request).send()).video_note.unwrap();

    let downloaded_video_note = run_one(rutebot.prepare_api_request(GetFile::new(&response.file_id)).send().and_then(move |x| rutebot.download_file(&x.file_path.unwrap())));
    assert_eq!(downloaded_video_note.len(), video_note_size);
}

#[test]
pub fn send_media_group_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_note_content = Vec::new();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg").unwrap().read_to_end(&mut photo_content).unwrap();
    File::open("./tests/sample_video_note.mp4").unwrap().read_to_end(&mut video_note_content).unwrap();
    let request = SendMediaGroup::new(
        chat_id,
        vec![
            InputMediaPhotoOrVideo::Video(InputMediaVideo::new(FileKind::InputFile {
                name: "video",
                content: video_note_content,
            })),
            InputMediaPhotoOrVideo::Photo(InputMediaPhoto::new(FileKind::InputFile {
                name: "photo",
                content: photo_content,
            }))
        ]);

    let response: Vec<Message> = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.len(), 2);
}

#[test]
pub fn send_location_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation::new(chat_id, 63.4, 32.2);

    let response: Message = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.location.is_some(), true);
}

#[test]
pub fn edit_location_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation {
        live_period: Some(60),
        ..SendLocation::new(chat_id, 63.4, 32.2)
    };
    let location: Message = run_one(rutebot.prepare_api_request(request).send());
    let edit_request = EditLiveLocation::new_chat(chat_id, location.message_id, 63.2, 32.1);

    if let EditedLiveLocation::Message(message) = run_one(rutebot.prepare_api_request(edit_request).send()) {
        assert_eq!(message.location.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[test]
pub fn stop_location_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation {
        live_period: Some(60),
        ..SendLocation::new(chat_id, 63.4, 32.2)
    };
    let location: Message = run_one(rutebot.prepare_api_request(request).send());
    let stop_request = StopLiveLocation::new_chat(chat_id, location.message_id);

    if let EditedLiveLocation::Message(message) = run_one(rutebot.prepare_api_request(stop_request).send()) {
        assert_eq!(message.location.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[test]
pub fn send_venue_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendVenue::new(chat_id, 63.4, 32.2, "test_title", "test_address");

    let venue: Venue = run_one(rutebot.prepare_api_request(request).send()).venue.unwrap();

    assert_eq!(venue.address, "test_address");
    assert_eq!(venue.title, "test_title");
}

#[test]
pub fn send_contact_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendContact::new(chat_id, "+79506470372", "imya");

    let contact: Contact = run_one(rutebot.prepare_api_request(request).send()).contact.unwrap();

    assert_eq!(contact.phone_number, "+79506470372");
    assert_eq!(contact.first_name, "imya");
}

#[test]
pub fn send_poll_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendPoll::new(chat_id, "to be or not to be", &["to be", "do not to be", "see results"]);

    let poll: Poll = run_one(rutebot.prepare_api_request(request).send()).poll.unwrap();

    assert_eq!(&poll.question, "to be or not to be");
}

#[test]
pub fn get_user_profile_photos_works() {
    let rutebot = common::create_client();
    let user_id = common::get_user_id();
    let request = GetUserProfilePhotos::new(user_id);

    let photos: UserProfilePhotos = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(photos.total_count, photos.photos.len() as i64)
}

#[test]
pub fn export_chat_invite_link_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = ExportChatInviteLink::new(chat_id);

    let _photos: String = run_one(rutebot.prepare_api_request(request).send());
}

#[test]
pub fn set_chat_photo_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg").unwrap().read_to_end(&mut photo_content).unwrap();
    let request = SetChatPhoto::new(chat_id, photo_content);

    let is_changed: bool = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(is_changed, true);
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