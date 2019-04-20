[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Leonqn/rutebot/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rutebot.svg)](https://crates.io/crates/rutebot)
[![doc.rs](https://docs.rs/rutebot/badge.svg)](https://docs.rs/rutebot)

# rutebot
Telegram bot api bindings for rust programming language. For details see [docs](https://docs.rs/rutebot)
## Example
Simple echo bot. It listens all incoming messages and echos text messages, on other messages it replies with text "I can echo only text message".
```rust
use hyper::rt::{Future, Stream};

use rutebot::client::Rutebot;
use rutebot::requests::get_updates::GetUpdates;
use rutebot::requests::send_message::send_text::SendText;
use rutebot::responses::{Message, Update};

fn main() {
    let rutebot = Rutebot::new("689791636:AAFYUZPb-h866CV5K6iXAR7d6cD2xJhIs3s");
    let get_updates =
        GetUpdates {
            timeout: Some(20),
            ..GetUpdates::new()
        };
    let updates = rutebot.incoming_updates(get_updates)
        .then(Ok)
        .for_each(move |x| {
            let reply_msg_request =
                match x {
                    Ok(Update { message: Some(Message { message_id, ref chat, text: Some(ref text), .. }), .. }) => {
                        let request =
                            SendText::new_reply(chat.id, text, message_id);
                        Some(request)
                    }
                    Ok(Update { message: Some(Message { message_id, ref chat, .. }), .. }) => {
                        let request = SendText::new_reply(chat.id, "This is not text...", message_id);
                        Some(request)
                    }
                    Err(e) => {
                        println!("Got error while getting updates {:?}", e);
                        None
                    }
                    _ => None
                };
            if let Some(reply) = reply_msg_request {
                let send_future = rutebot.prepare_api_request(reply)
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

