# rutebot
[![Build Status](https://travis-ci.org/Leonqn/rutebot.svg?branch=master)](https://travis-ci.org/Leonqn/rutebot)
[![Crates.io](https://img.shields.io/crates/v/rutebot.svg)](https://crates.io/crates/rutebot)
[![doc.rs](https://docs.rs/rutebot/badge.svg)](https://docs.rs/rutebot)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Leonqn/rutebot/LICENSE)

**Ru**st **Te**legram **Bot**. A framework offering [Telegram Bot API](https://core.telegram.org/bots/api) bindings for the Rust programming language.

For details see the [docs](https://docs.rs/rutebot).

## Example
A simple greetings bot. Replies to all messages with text "Hello %USERNAME%"

You can run the following example with `cargo run --example simplebot`.


```rust
use std::env;

use futures_util::StreamExt;
use rutebot::client::Rutebot;
use rutebot::requests::{SendMessage};
use rutebot::responses::Update;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token_env = env::var_os("TELEGRAM_TOKEN")
        .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
    let token = token_env.to_string_lossy();

    let rutebot = Rutebot::new(token);
    let mut updates_stream = Box::pin(rutebot.incoming_updates(None, None));
    while let Some(update) = updates_stream.next().await.transpose()? {
        let create_reply_request = |update: Update| {
            let message = update.message?;
            let response_message = format!("Hello {}", message.from?.first_name);
            let reply =
                SendMessage::new_reply(message.chat.id, &response_message, message.message_id);
            Some(rutebot.prepare_api_request(reply))
        };

        if let Some(reply) = create_reply_request(update) {
            tokio::spawn(reply.send());
        }
    }
    Ok(())
}

```