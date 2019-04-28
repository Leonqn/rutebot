use futures::future::Future;
use pretty_assertions::assert_eq;

use rutebot::requests::delete_chat_photo::DeleteChatPhoto;
use rutebot::requests::delete_message::DeleteMessage;
use rutebot::requests::edit_live_location::EditLiveLocation;
use rutebot::requests::edit_message_caption::EditMessageCaption;
use rutebot::requests::edit_message_media::EditMessageMedia;
use rutebot::requests::edit_message_text::EditMessageText;
use rutebot::requests::export_chat_invite_link::ExportChatInviteLink;
use rutebot::requests::forward_message::ForwardMessage;
use rutebot::requests::get_chat::GetChat;
use rutebot::requests::get_chat_administrators::GetChatAdministrators;
use rutebot::requests::get_chat_members_count::GetChatMembersCount;
use rutebot::requests::get_file::GetFile;
use rutebot::requests::get_me::GetMe;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::get_user_profile_photos::GetUserProfilePhotos;
use rutebot::requests::pin_chat_message::PinChatMessage;
use rutebot::requests::send_animation::SendAnimation;
use rutebot::requests::send_audio::SendAudio;
use rutebot::requests::send_chat_action::{ChatAction, SendChatAction};
use rutebot::requests::send_contact::SendContact;
use rutebot::requests::send_document::SendDocument;
use rutebot::requests::send_location::SendLocation;
use rutebot::requests::send_media_group::{InputMediaPhotoOrVideo, SendMediaGroup};
use rutebot::requests::send_photo::SendPhoto;
use rutebot::requests::send_poll::SendPoll;
use rutebot::requests::send_message::SendMessage;
use rutebot::requests::send_venue::SendVenue;
use rutebot::requests::send_video::SendVideo;
use rutebot::requests::send_video_note::SendVideoNote;
use rutebot::requests::send_voice::SendVoice;
use rutebot::requests::set_chat_description::SetChatDescription;
use rutebot::requests::set_chat_photo::SetChatPhoto;
use rutebot::requests::set_chat_title::SetChatTitle;
use rutebot::requests::stop_live_location::StopLiveLocation;
use rutebot::requests::stop_poll::StopPoll;
use rutebot::requests::unpin_chat_message::UnpinChatMessage;
use rutebot::requests::{
    FileKind, InlineKeyboard, InlineKeyboardButton, InputMedia, InputMediaPhoto, InputMediaVideo,
    ParseMode, ReplyMarkup,
};
use rutebot::responses::{
    Audio, Chat, ChatMember, Contact, Document, EditedMessage, Message, MessageEntityValue, Poll,
    Update, User, UserProfilePhotos, Venue, Video, VideoNote, Voice,
};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use crate::common::run_one;

mod common;

#[test]
pub fn get_me_works() {
    let rutebot = common::create_client();

    let response: User = run_one(rutebot.prepare_api_request(GetMe).send());

    assert_eq!(response.is_bot, true);
}

#[test]
pub fn send_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage::new(chat_id, "Some text"))
            .send(),
    );

    assert_eq!(response.text.unwrap(), "Some text");
}

#[test]
pub fn forward_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let sent_msg: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage::new(chat_id, "test"))
            .send(),
    );

    let response: Message = run_one(
        rutebot
            .prepare_api_request(ForwardMessage::new(chat_id, chat_id, sent_msg.message_id))
            .send(),
    );

    assert_eq!(sent_msg.text, response.text);
}

#[test]
pub fn send_chat_action_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: bool = run_one(
        rutebot
            .prepare_api_request(SendChatAction::new(chat_id, ChatAction::Typing))
            .send(),
    );

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
        }]],
    };
    let request = SendDocument {
        caption: Some("random file"),
        reply_markup: Some(ReplyMarkup::InlineKeyboard(keyboard)),
        ..SendDocument::new(
            chat_id,
            FileKind::InputFile {
                name: "superfile",
                content: vec![1, 2, 3, 4, 5],
            },
        )
    };

    let response: Document = run_one(rutebot.prepare_api_request(request).send())
        .document
        .unwrap();

    let downloaded_document = run_one(
        rutebot
            .prepare_api_request(GetFile::new(&response.file_id))
            .send()
            .and_then(move |x| rutebot.download_file(&x.file_path.unwrap())),
    );
    assert_eq!(response.file_size, Some(5));
    assert_eq!(response.file_name, Some("superfile".to_owned()));
    assert_eq!(downloaded_document, vec![1, 2, 3, 4, 5]);
}

#[test]
pub fn send_photo_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let request = SendPhoto::new(
        chat_id,
        FileKind::InputFile {
            name: "superphoto",
            content: photo_content,
        },
    );

    let response: Message = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.photo.is_some(), true);
}

#[test]
pub fn send_audio_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut audio_content = Vec::new();
    File::open("./tests/Dark_Tranquility.mp3")
        .unwrap()
        .read_to_end(&mut audio_content)
        .unwrap();
    let audio_size = audio_content.len();
    let request = SendAudio {
        performer: Some("Dark_Tranquility"),
        ..SendAudio::new(
            chat_id,
            FileKind::InputFile {
                name: "superaudio",
                content: audio_content,
            },
        )
    };

    let response: Audio = run_one(rutebot.prepare_api_request(request).send())
        .audio
        .unwrap();

    let downloaded_audio = run_one(
        rutebot
            .prepare_api_request(GetFile::new(&response.file_id))
            .send()
            .and_then(move |x| rutebot.download_file(&x.file_path.unwrap())),
    );
    assert_eq!(downloaded_audio.len(), audio_size);
}

#[test]
pub fn send_video_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_content = Vec::new();
    File::open("./tests/sample_video.mp4")
        .unwrap()
        .read_to_end(&mut video_content)
        .unwrap();
    let video_size = video_content.len();
    let request = SendVideo::new(
        chat_id,
        FileKind::InputFile {
            name: "supervideo",
            content: video_content,
        },
    );

    let response: Video = run_one(rutebot.prepare_api_request(request).send())
        .video
        .unwrap();

    let downloaded_video = run_one(
        rutebot
            .prepare_api_request(GetFile::new(&response.file_id))
            .send()
            .and_then(move |x| rutebot.download_file(&x.file_path.unwrap())),
    );
    assert_eq!(downloaded_video.len(), video_size);
}

#[test]
pub fn send_animation_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut gif_content = Vec::new();
    File::open("./tests/sample_gif.gif")
        .unwrap()
        .read_to_end(&mut gif_content)
        .unwrap();
    let request = SendAnimation {
        width: Some(808),
        height: Some(538),
        ..SendAnimation::new(
            chat_id,
            FileKind::InputFile {
                name: "supergif",
                content: gif_content,
            },
        )
    };

    let response: Message = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(response.animation.is_some(), true);
}

#[test]
pub fn send_voice_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut voice_content = Vec::new();
    File::open("./tests/sample_voice.ogg")
        .unwrap()
        .read_to_end(&mut voice_content)
        .unwrap();
    let voice_size = voice_content.len();
    let request = SendVoice::new(
        chat_id,
        FileKind::InputFile {
            name: "supervoice",
            content: voice_content,
        },
    );

    let response: Voice = run_one(rutebot.prepare_api_request(request).send())
        .voice
        .unwrap();

    let downloaded_voice = run_one(
        rutebot
            .prepare_api_request(GetFile::new(&response.file_id))
            .send()
            .and_then(move |x| rutebot.download_file(&x.file_path.unwrap())),
    );
    assert_eq!(downloaded_voice.len(), voice_size);
}

#[test]
pub fn send_video_note_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_note_content = Vec::new();
    File::open("./tests/sample_video_note.mp4")
        .unwrap()
        .read_to_end(&mut video_note_content)
        .unwrap();
    let video_note_size = video_note_content.len();
    let request = SendVideoNote::new(
        chat_id,
        FileKind::InputFile {
            name: "supervideonote",
            content: video_note_content,
        },
    );

    let response: VideoNote = run_one(rutebot.prepare_api_request(request).send())
        .video_note
        .unwrap();

    let downloaded_video_note = run_one(
        rutebot
            .prepare_api_request(GetFile::new(&response.file_id))
            .send()
            .and_then(move |x| rutebot.download_file(&x.file_path.unwrap())),
    );
    assert_eq!(downloaded_video_note.len(), video_note_size);
}

#[test]
pub fn send_media_group_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut video_note_content = Vec::new();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    File::open("./tests/sample_video_note.mp4")
        .unwrap()
        .read_to_end(&mut video_note_content)
        .unwrap();
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
            })),
        ],
    );

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
    let edit_request = EditLiveLocation::new_message(chat_id, location.message_id, 63.2, 32.1);

    if let EditedMessage::Message(message) =
        run_one(rutebot.prepare_api_request(edit_request).send())
    {
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

    if let EditedMessage::Message(message) =
        run_one(rutebot.prepare_api_request(stop_request).send())
    {
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

    let venue: Venue = run_one(rutebot.prepare_api_request(request).send())
        .venue
        .unwrap();

    assert_eq!(venue.address, "test_address");
    assert_eq!(venue.title, "test_title");
}

#[test]
pub fn send_contact_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendContact::new(chat_id, "+79506470372", "imya");

    let contact: Contact = run_one(rutebot.prepare_api_request(request).send())
        .contact
        .unwrap();

    assert_eq!(contact.phone_number, "+79506470372");
    assert_eq!(contact.first_name, "imya");
}

#[test]
pub fn send_poll_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendPoll::new(
        chat_id,
        "to be or not to be",
        &["to be", "do not to be", "see results"],
    );

    let poll: Poll = run_one(rutebot.prepare_api_request(request).send())
        .poll
        .unwrap();

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
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let request = SetChatPhoto::new(chat_id, photo_content);

    let is_changed: bool = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(is_changed, true);
}

#[test]
pub fn delete_chat_photo_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let set_request = SetChatPhoto::new(chat_id, photo_content);
    run_one(rutebot.prepare_api_request(set_request).send());
    let request = DeleteChatPhoto::new(chat_id);

    let is_deleted = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(is_deleted, true);
}

#[test]
pub fn set_chat_title_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SetChatTitle::new(chat_id, "new_title");

    let title_set = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(title_set, true);
}

#[test]
pub fn set_chat_description_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let new_description = format!("new description. {:?}", Instant::now());
    let request = SetChatDescription::new_description(chat_id, &new_description);

    let description_set = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(description_set, true);
}

#[test]
pub fn pin_chat_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let new_message: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage::new(chat_id, "Some text"))
            .send(),
    );
    let request = PinChatMessage {
        disable_notification: true,
        ..PinChatMessage::new(chat_id, new_message.message_id)
    };

    let pinned = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(pinned, true);
}

#[test]
pub fn unpin_chat_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = UnpinChatMessage::new(chat_id);

    let pinned = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(pinned, true);
}

#[test]
pub fn get_chat_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChat::new(chat_id);

    let chat: Chat = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(chat.id, chat_id);
}

#[test]
pub fn get_chat_administrators_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChatAdministrators::new(chat_id);

    let chat: Vec<ChatMember> = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(chat.len() > 0, true);
}

#[test]
pub fn get_chat_members_count_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChatMembersCount::new(chat_id);

    let members_count: i64 = run_one(rutebot.prepare_api_request(request).send());

    assert_eq!(members_count > 0, true);
}

#[test]
pub fn edit_message_text_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let text_message: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage::new(chat_id, "Some text"))
            .send(),
    );
    let edit_request = EditMessageText::new_message(chat_id, text_message.message_id, "new text");

    if let EditedMessage::Message(message) =
        run_one(rutebot.prepare_api_request(edit_request).send())
    {
        assert_eq!(message.text.unwrap(), "new text");
    } else {
        panic!("Returned true.");
    }
}

#[test]
pub fn edit_message_caption_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut gif_content = Vec::new();
    File::open("./tests/sample_gif.gif")
        .unwrap()
        .read_to_end(&mut gif_content)
        .unwrap();
    let request = SendAnimation {
        width: Some(808),
        height: Some(538),
        caption: Some("old caption"),
        ..SendAnimation::new(
            chat_id,
            FileKind::InputFile {
                name: "supergif",
                content: gif_content,
            },
        )
    };

    let animation: Message = run_one(rutebot.prepare_api_request(request).send());
    let edit_request =
        EditMessageCaption::new_message(chat_id, animation.message_id, "new caption");

    if let EditedMessage::Message(message) =
        run_one(rutebot.prepare_api_request(edit_request).send())
    {
        assert_eq!(&message.caption.unwrap(), "new caption");
    } else {
        panic!("Returned true.");
    }
}

#[test]
pub fn edit_message_media_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut old_video = Vec::new();
    let mut new_video = Vec::new();
    File::open("./tests/sample_video_note.mp4")
        .unwrap()
        .read_to_end(&mut old_video)
        .unwrap();
    File::open("./tests/sample_video.mp4")
        .unwrap()
        .read_to_end(&mut new_video)
        .unwrap();
    let request = SendVideo::new(
        chat_id,
        FileKind::InputFile {
            name: "supervideo",
            content: old_video,
        },
    );
    let response: Message = run_one(rutebot.prepare_api_request(request).send());
    let edit_video = EditMessageMedia::new_message(
        chat_id,
        response.message_id,
        InputMedia::Video(InputMediaVideo::new(FileKind::InputFile {
            name: "supervideo",
            content: new_video,
        })),
    );

    if let EditedMessage::Message(message) = run_one(rutebot.prepare_api_request(edit_video).send())
    {
        assert_eq!(message.video.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[test]
pub fn stop_poll_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendPoll::new(
        chat_id,
        "to be or not to be",
        &["to be", "do not to be", "see results"],
    );
    let msg_with_poll: Message = run_one(rutebot.prepare_api_request(request).send());
    let stop_poll_request = StopPoll::new(chat_id, msg_with_poll.message_id);

    let response: Poll = run_one(rutebot.prepare_api_request(stop_poll_request).send());

    assert_eq!(&response.question, "to be or not to be");
}

#[test]
pub fn delete_message_works() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let response: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage::new(chat_id, "Some text"))
            .send(),
    );
    let delete_message_request = DeleteMessage::new(chat_id, response.message_id);

    let response: bool = run_one(rutebot.prepare_api_request(delete_message_request).send());

    assert_eq!(response, true);
}

#[test]
pub fn message_entity_values_extracted_correctly() {
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let weird_text =
        "великий и могучий: [экзамл.ком](http://example.com/) очень могучий и великий";

    let response: Message = run_one(
        rutebot
            .prepare_api_request(SendMessage {
                parse_mode: Some(ParseMode::Markdown),
                ..SendMessage::new(chat_id, weird_text)
            })
            .send(),
    );
    let text = &response.text.unwrap();
    let entities = response.entities.unwrap();
    let values = entities
        .iter()
        .filter_map(|x| x.extract_value(text))
        .collect::<Vec<MessageEntityValue>>();
    let message_entity = values.first();

    match message_entity {
        Some(MessageEntityValue::TextLink { link, text }) => {
            assert_eq!(link.as_str(), "http://example.com/");
            assert_eq!(text.as_str(), "экзамл.ком");
        }
        x => panic!("wrong message entity: {:?}", x),
    }
}
