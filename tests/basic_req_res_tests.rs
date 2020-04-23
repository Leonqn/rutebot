use pretty_assertions::assert_eq;

use rutebot::{
    requests::{
        ChatAction, DeleteChatPhoto, DeleteMessage, EditLiveLocation, EditMessageCaption,
        EditMessageMedia, EditMessageText, ExportChatInviteLink, FileKind, ForwardMessage, GetChat,
        GetChatAdministrators, GetChatMembersCount, GetFile, GetMe, GetUpdates,
        GetUserProfilePhotos, InlineKeyboard, InlineKeyboardButton, InputMedia, InputMediaPhoto,
        InputMediaPhotoOrVideo, InputMediaVideo, ParseMode, PinChatMessage, ReplyMarkup,
        SendAnimation, SendAudio, SendChatAction, SendContact, SendDocument, SendLocation,
        SendMediaGroup, SendMessage, SendPhoto, SendPoll, SendVenue, SendVideo, SendVideoNote,
        SendVoice, SetChatDescription, SetChatPhoto, SetChatTitle, StopLiveLocation, StopPoll,
        UnpinChatMessage,
    },
    responses::{
        Audio, Chat, ChatMember, Contact, Document, EditedMessage, Message, MessageEntityValue,
        Poll, Update, User, UserProfilePhotos, Venue, Video, VideoNote, Voice,
    },
};
use std::{fs::File, io::Read, time::Instant};
use crate::common::MUTEX;

mod common;

#[tokio::test]
async fn get_me_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();

    let response: User = rutebot.prepare_api_request(GetMe).send().await.unwrap();

    assert_eq!(response.is_bot, true);
}

#[tokio::test]
async fn send_message_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: Message = rutebot
        .prepare_api_request(SendMessage::new(chat_id, "Some text"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.text.unwrap(), "Some text");
}

#[tokio::test]
async fn forward_message_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let sent_msg: Message = rutebot
        .prepare_api_request(SendMessage::new(chat_id, "test"))
        .send()
        .await
        .unwrap();

    let response: Message = rutebot
        .prepare_api_request(ForwardMessage::new(chat_id, chat_id, sent_msg.message_id))
        .send()
        .await
        .unwrap();

    assert_eq!(sent_msg.text, response.text);
}

#[tokio::test]
async fn send_chat_action_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();

    let response: bool = rutebot
        .prepare_api_request(SendChatAction::new(chat_id, ChatAction::Typing))
        .send()
        .await
        .unwrap();

    assert_eq!(response, true);
}

#[tokio::test]
async fn get_updates_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();

    let _response: Vec<Update> = rutebot
        .prepare_api_request(GetUpdates::new())
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn send_document_works() {
    MUTEX.lock().unwrap();
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
                thumb: None,
            },
        )
    };

    let response: Document = rutebot
        .prepare_api_request(request)
        .send()
        .await
        .unwrap()
        .document
        .unwrap();
    let file_handle = rutebot
        .prepare_api_request(GetFile::new(&response.file_id))
        .send()
        .await
        .unwrap();
    let downloaded_document = rutebot
        .download_file(&file_handle.file_path.unwrap())
        .await
        .unwrap();

    assert_eq!(response.file_size, Some(5));
    assert_eq!(response.file_name, Some("superfile".to_owned()));
    assert_eq!(downloaded_document, vec![1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn send_document_with_thumb_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let request = SendDocument {
        caption: Some("random file with thumb"),
        ..SendDocument::new(
            chat_id,
            FileKind::InputFile {
                name: "superfile",
                content: vec![1, 2, 3, 4, 5],
                thumb: Some(photo_content),
            },
        )
    };

    let response: Document = rutebot
        .prepare_api_request(request)
        .send()
        .await
        .unwrap()
        .document
        .unwrap();

    assert_eq!(response.file_size, Some(5));
    assert_eq!(response.file_name, Some("superfile".to_owned()));
    assert_eq!(response.thumb.is_some(), true);
}

#[tokio::test]
async fn send_photo_works() {
    MUTEX.lock().unwrap();
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
            thumb: None,
        },
    );

    let response: Message = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(response.photo.is_some(), true);
}

#[tokio::test]
async fn send_audio_works() {
    MUTEX.lock().unwrap();
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
                thumb: None,
            },
        )
    };

    let response: Audio = rutebot
        .prepare_api_request(request)
        .send()
        .await
        .unwrap()
        .audio
        .unwrap();
    let file_handle = rutebot
        .prepare_api_request(GetFile::new(&response.file_id))
        .send()
        .await
        .unwrap();
    let downloaded_audio = rutebot
        .download_file(&file_handle.file_path.unwrap())
        .await
        .unwrap();

    assert_eq!(downloaded_audio.len(), audio_size);
}

#[tokio::test]
async fn send_video_works() {
    MUTEX.lock().unwrap();
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
            thumb: None,
        },
    );

    let response: Video = rutebot
        .prepare_api_request(request)
        .send()
        .await
        .unwrap()
        .video
        .unwrap();
    let file_handle = rutebot
        .prepare_api_request(GetFile::new(&response.file_id))
        .send()
        .await
        .unwrap();
    let downloaded_video = rutebot
        .download_file(&file_handle.file_path.unwrap())
        .await
        .unwrap();

    assert_eq!(downloaded_video.len(), video_size);
}

#[tokio::test]
async fn send_animation_works() {
    MUTEX.lock().unwrap();
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
                thumb: None,
            },
        )
    };

    let response: Message = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(response.animation.is_some(), true);
}

#[tokio::test]
async fn send_voice_works() {
    MUTEX.lock().unwrap();
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
            thumb: None,
        },
    );

    let response: Voice = (rutebot.prepare_api_request(request).send().await.unwrap())
        .voice
        .unwrap();
    let file_handle = rutebot
        .prepare_api_request(GetFile::new(&response.file_id))
        .send()
        .await
        .unwrap();
    let downloaded_voice = rutebot
        .download_file(&file_handle.file_path.unwrap())
        .await
        .unwrap();

    assert_eq!(downloaded_voice.len(), voice_size);
}

#[tokio::test]
async fn send_video_note_works() {
    MUTEX.lock().unwrap();
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
            thumb: None,
        },
    );

    let response: VideoNote = (rutebot.prepare_api_request(request).send().await.unwrap())
        .video_note
        .unwrap();
    let file_handle = rutebot
        .prepare_api_request(GetFile::new(&response.file_id))
        .send()
        .await
        .unwrap();
    let downloaded_video_note = rutebot
        .download_file(&file_handle.file_path.unwrap())
        .await
        .unwrap();

    assert_eq!(downloaded_video_note.len(), video_note_size);
}

#[tokio::test]
async fn send_media_group_works() {
    MUTEX.lock().unwrap();
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
                thumb: None,
            })),
            InputMediaPhotoOrVideo::Photo(InputMediaPhoto::new(FileKind::InputFile {
                name: "photo",
                content: photo_content,
                thumb: None,
            })),
        ],
    );

    let response: Vec<Message> = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(response.len(), 2);
}

#[tokio::test]
async fn send_location_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation::new(chat_id, 63.4, 32.2);

    let response: Message = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(response.location.is_some(), true);
}

#[tokio::test]
async fn edit_location_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation {
        live_period: Some(60),
        ..SendLocation::new(chat_id, 63.4, 32.2)
    };
    let location: Message = rutebot.prepare_api_request(request).send().await.unwrap();
    let edit_request = EditLiveLocation::new_message(chat_id, location.message_id, 63.2, 32.1);

    if let EditedMessage::Message(message) = rutebot
        .prepare_api_request(edit_request)
        .send()
        .await
        .unwrap()
    {
        assert_eq!(message.location.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[tokio::test]
async fn stop_location_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendLocation {
        live_period: Some(60),
        ..SendLocation::new(chat_id, 63.4, 32.2)
    };
    let location: Message = rutebot.prepare_api_request(request).send().await.unwrap();
    let stop_request = StopLiveLocation::new_chat(chat_id, location.message_id);

    if let EditedMessage::Message(message) = rutebot
        .prepare_api_request(stop_request)
        .send()
        .await
        .unwrap()
    {
        assert_eq!(message.location.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[tokio::test]
async fn send_venue_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendVenue::new(chat_id, 63.4, 32.2, "test_title", "test_address");

    let venue: Venue = (rutebot.prepare_api_request(request).send().await.unwrap())
        .venue
        .unwrap();

    assert_eq!(venue.address, "test_address");
    assert_eq!(venue.title, "test_title");
}

#[tokio::test]
async fn send_contact_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendContact::new(chat_id, "+79506470372", "imya");

    let contact: Contact = (rutebot.prepare_api_request(request).send().await.unwrap())
        .contact
        .unwrap();

    assert_eq!(contact.phone_number, "+79506470372");
    assert_eq!(contact.first_name, "imya");
}

#[tokio::test]
async fn send_poll_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendPoll::new(
        chat_id,
        "to be or not to be",
        &["to be", "not to be", "see results"],
    );

    let poll: Poll = (rutebot.prepare_api_request(request).send().await.unwrap())
        .poll
        .unwrap();

    assert_eq!(&poll.question, "to be or not to be");
}

#[tokio::test]
async fn get_user_profile_photos_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let user_id = common::get_user_id();
    let request = GetUserProfilePhotos::new(user_id);

    let photos: UserProfilePhotos = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(photos.total_count, photos.photos.len() as i64)
}

#[tokio::test]
async fn export_chat_invite_link_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = ExportChatInviteLink::new(chat_id);

    let _photos: String = rutebot.prepare_api_request(request).send().await.unwrap();
}

#[tokio::test]
async fn set_chat_photo_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let request = SetChatPhoto::new(chat_id, photo_content);

    let is_changed: bool = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(is_changed, true);
}

#[tokio::test]
async fn delete_chat_photo_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut photo_content = Vec::new();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let set_request = SetChatPhoto::new(chat_id, photo_content);
    (rutebot
        .prepare_api_request(set_request)
        .send()
        .await
        .unwrap());
    let request = DeleteChatPhoto::new(chat_id);

    let is_deleted = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(is_deleted, true);
}

#[tokio::test]
async fn set_chat_title_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SetChatTitle::new(chat_id, "new_title");

    let title_set = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(title_set, true);
}

#[tokio::test]
async fn set_chat_description_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let new_description = format!("new description. {:?}", Instant::now());
    let request = SetChatDescription::new_description(chat_id, &new_description);

    let description_set = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(description_set, true);
}

#[tokio::test]
async fn pin_chat_message_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let new_message: Message = rutebot
        .prepare_api_request(SendMessage::new(chat_id, "Some text"))
        .send()
        .await
        .unwrap();
    let request = PinChatMessage {
        disable_notification: true,
        ..PinChatMessage::new(chat_id, new_message.message_id)
    };

    let pinned = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(pinned, true);
}

#[tokio::test]
async fn unpin_chat_message_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = UnpinChatMessage::new(chat_id);

    let pinned = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(pinned, true);
}

#[tokio::test]
async fn get_chat_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChat::new(chat_id);

    let chat: Chat = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(chat.id, chat_id);
}

#[tokio::test]
async fn get_chat_administrators_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChatAdministrators::new(chat_id);

    let chat: Vec<ChatMember> = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(chat.is_empty(), false);
}

#[tokio::test]
async fn get_chat_members_count_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = GetChatMembersCount::new(chat_id);

    let members_count: i64 = rutebot.prepare_api_request(request).send().await.unwrap();

    assert_eq!(members_count > 0, true);
}

#[tokio::test]
async fn edit_message_text_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let text_message: Message = rutebot
        .prepare_api_request(SendMessage::new(chat_id, "Some text"))
        .send()
        .await
        .unwrap();
    let edit_request = EditMessageText::new_message(chat_id, text_message.message_id, "new text");

    if let EditedMessage::Message(message) = rutebot
        .prepare_api_request(edit_request)
        .send()
        .await
        .unwrap()
    {
        assert_eq!(message.text.unwrap(), "new text");
    } else {
        panic!("Returned true.");
    }
}

#[tokio::test]
async fn edit_message_caption_works() {
    MUTEX.lock().unwrap();
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
                thumb: None,
            },
        )
    };

    let animation: Message = rutebot.prepare_api_request(request).send().await.unwrap();
    let edit_request =
        EditMessageCaption::new_message(chat_id, animation.message_id, "new caption");

    if let EditedMessage::Message(message) = rutebot
        .prepare_api_request(edit_request)
        .send()
        .await
        .unwrap()
    {
        assert_eq!(&message.caption.unwrap(), "new caption");
    } else {
        panic!("Returned true.");
    }
}

#[tokio::test]
async fn edit_message_media_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let mut old_video = Vec::new();
    let mut new_video = Vec::new();
    let mut photo_content = Vec::new();
    File::open("./tests/sample_video_note.mp4")
        .unwrap()
        .read_to_end(&mut old_video)
        .unwrap();
    File::open("./tests/sample_video.mp4")
        .unwrap()
        .read_to_end(&mut new_video)
        .unwrap();
    File::open("./tests/photo_test.jpg")
        .unwrap()
        .read_to_end(&mut photo_content)
        .unwrap();
    let request = SendVideo::new(
        chat_id,
        FileKind::InputFile {
            name: "supervideo",
            content: old_video,
            thumb: None,
        },
    );
    let response: Message = rutebot.prepare_api_request(request).send().await.unwrap();
    let edit_video = EditMessageMedia::new_message(
        chat_id,
        response.message_id,
        InputMedia::Video(InputMediaVideo::new(FileKind::InputFile {
            name: "supervideo",
            content: new_video,
            thumb: Some(photo_content),
        })),
    );

    if let EditedMessage::Message(message) = rutebot
        .prepare_api_request(edit_video)
        .send()
        .await
        .unwrap()
    {
        assert_eq!(message.video.is_some(), true);
        assert_eq!(message.video.unwrap().thumb.is_some(), true);
    } else {
        panic!("Returned true.");
    }
}

#[tokio::test]
async fn stop_poll_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let request = SendPoll::new(
        chat_id,
        "to be or not to be",
        &["to be", "do not to be", "see results"],
    );
    let msg_with_poll: Message = rutebot.prepare_api_request(request).send().await.unwrap();
    let stop_poll_request = StopPoll::new(chat_id, msg_with_poll.message_id);

    let response: Poll = rutebot
        .prepare_api_request(stop_poll_request)
        .send()
        .await
        .unwrap();

    assert_eq!(&response.question, "to be or not to be");
}

#[tokio::test]
async fn delete_message_works() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let response: Message = rutebot
        .prepare_api_request(SendMessage::new(chat_id, "Some text"))
        .send()
        .await
        .unwrap();
    let delete_message_request = DeleteMessage::new(chat_id, response.message_id);

    let response: bool = rutebot
        .prepare_api_request(delete_message_request)
        .send()
        .await
        .unwrap();

    assert_eq!(response, true);
}

#[tokio::test]
async fn message_entity_values_extracted_correctly() {
    MUTEX.lock().unwrap();
    let rutebot = common::create_client();
    let chat_id = common::get_chat_id();
    let weird_text = "великий и могучий: [экзамл.ком](http://example.com/) очень могучий и великий";

    let response: Message = rutebot
        .prepare_api_request(SendMessage {
            parse_mode: Some(ParseMode::Markdown),
            ..SendMessage::new(chat_id, weird_text)
        })
        .send()
        .await
        .unwrap();
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
