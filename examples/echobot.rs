use std::env;

use futures::future::Future;
use futures::stream::Stream;
use rutebot::client::Rutebot;
use rutebot::requests::{GetUpdates, SendMessage};
use rutebot::responses::{Message, Update};

fn main() {
    let token_env = env::var_os("TELEGRAM_TOKEN")
        .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
    let token = token_env.to_string_lossy();

    let rutebot = Rutebot::new(token);
    let get_updates = GetUpdates::new_with_timeout(20);
    let updates = rutebot
        .incoming_updates(get_updates)
        .then(Ok)
        .for_each(move |update| {
            match update {
                Ok(Update {
                    message:
                        Some(Message {
                            message_id,
                            ref chat,
                            text: Some(ref text),
                            ..
                        }),
                    ..
                }) => {
                    let request = SendMessage::new_reply(chat.id, text, message_id);
                    let send_future = rutebot
                        .prepare_api_request(request)
                        .send()
                        .map(|_| ())
                        .map_err(|x| println!("Got error while sending message: {:?}", x));
                    tokio::spawn(send_future);
                }
                Err(e) => {
                    println!("Got error while getting updates {:?}", e);
                }
                _ => (),
            };
            Ok(())
        });

    tokio::run(updates);
}
