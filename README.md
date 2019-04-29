# rutebot
[![Build Status](https://travis-ci.org/Leonqn/rutebot.svg?branch=master)](https://travis-ci.org/Leonqn/rutebot)
[![Crates.io](https://img.shields.io/crates/v/rutebot.svg)](https://crates.io/crates/rutebot)
[![doc.rs](https://docs.rs/rutebot/badge.svg)](https://docs.rs/rutebot)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Leonqn/rutebot/LICENSE)

**Ru**st **Te**legram **Bot**. A framework offering [Telegram Bot API](https://core.telegram.org/bots/api) bindings for the Rust programming language.

For details see the [docs](https://docs.rs/rutebot).

## Example
A simple echo bot. Replies to text messages by echoing the message. Responds to other types of messages with "This is not text...".

You can run the following example with `cargo run --example echobot`.


```rust
use std::env;
use hyper::rt::{Future, Stream};

use rutebot::client::Rutebot;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::send_text::SendText;
use rutebot::responses::{Message, Update};

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
        .for_each(move |x| {
            let reply_msg_request = match x {
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
                    let request = SendText::new_reply(chat.id, text, message_id);
                    Some(request)
                }
                Ok(Update {
                    message:
                        Some(Message {
                            message_id,
                            ref chat,
                            ..
                        }),
                    ..
                }) => {
                    let request = SendText::new_reply(chat.id, "This is not text...", message_id);
                    Some(request)
                }
                Err(e) => {
                    println!("Got error while getting updates {:?}", e);
                    None
                }
                _ => None,
            };
            if let Some(reply) = reply_msg_request {
                let send_future = rutebot
                    .prepare_api_request(reply)
                    .send()
                    .map(|_| ())
                    .map_err(|x| println!("Got error while sending message: {:?}", x));
                hyper::rt::spawn(send_future);
            }
            Ok(())
        });

    hyper::rt::run(updates);
}
```