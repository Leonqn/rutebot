use hyper::rt::{Future, Stream};
use std::env;

use rutebot::client::Rutebot;
use rutebot::requests::{GetUpdates, Request, SendMessage};
use rutebot::responses::{Message, Update};

fn handle_update(update: Update) -> Option<impl Request> {
    match update {
        Update {
            message:
                Some(Message {
                    message_id,
                    chat,
                    text: Some(text),
                    ..
                }),
            ..
        } => {
            let request = SendMessage::new_reply(chat.id, text, message_id);
            Some(request)
        }
        Update {
            message:
                Some(Message {
                    message_id,
                    ref chat,
                    ..
                }),
            ..
        } => {
            let request =
                SendMessage::new_reply(chat.id, "This is not text...".to_owned(), message_id);
            Some(request)
        }
        _ => None,
    }
}

fn main() {
    let token_env = env::var_os("TELEGRAM_TOKEN")
        .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
    let token = token_env.to_string_lossy();

    let rutebot = Rutebot::new(token);
    let get_updates = GetUpdates {
        timeout: Some(20),
        ..GetUpdates::new()
    };
    let updates = rutebot
        .incoming_updates(get_updates)
        .then(Ok)
        .for_each(move |update_result| {
            match update_result {
                Ok(update) => {
                    if let Some(request) = handle_update(update) {
                        let send_future = rutebot
                            .prepare_api_request(request)
                            .send()
                            .map(|_| ())
                            .map_err(|x| println!("Got error while sending message: {:?}", x));
                        hyper::rt::spawn(send_future);
                    };
                }
                Err(e) => {
                    println!("Got error while getting updates {:?}", e);
                }
            }
            Ok(())
        });

    hyper::rt::run(updates);
}
