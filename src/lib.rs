//! Crate with bindings to telegram bot api. For usage details see example below and tests in `basic_req_res_tests.rs`.
//! You can find all implemented methods in `requests` module.
//!
//! # Example
//! Simple echo bot. It listens all incoming messages and echos text messages, on other messages it replies with text "I can echo only text message".
//! ```no_run
//! use hyper::rt::{Future, Stream};
//! use std::env;
//!
//! use rutebot::client::Rutebot;
//! use rutebot::requests::{GetUpdates, SendMessage};
//! use rutebot::responses::{Message, Update};
//!
//! fn main() {
//!     let token_env = env::var_os("TELEGRAM_TOKEN")
//!         .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
//!     let token = token_env.to_string_lossy();
//!
//!     let rutebot = Rutebot::new(token);
//!     let get_updates = GetUpdates {
//!         timeout: Some(20),
//!         ..GetUpdates::new()
//!     };
//!     let updates = rutebot
//!         .incoming_updates(get_updates)
//!         .then(Ok)
//!         .for_each(move |x| {
//!             let reply_msg_request = match x {
//!                 Ok(Update {
//!                     message:
//!                         Some(Message {
//!                             message_id,
//!                             ref chat,
//!                             text: Some(ref text),
//!                             ..
//!                         }),
//!                     ..
//!                 }) => {
//!                     let request = SendMessage::new_reply(chat.id, text, message_id);
//!                     Some(request)
//!                 }
//!                 Ok(Update {
//!                     message:
//!                         Some(Message {
//!                             message_id,
//!                             ref chat,
//!                             ..
//!                         }),
//!                     ..
//!                 }) => {
//!                     let request = SendMessage::new_reply(chat.id, "This is not text...", message_id);
//!                     Some(request)
//!                 }
//!                 Err(e) => {
//!                     println!("Got error while getting updates {:?}", e);
//!                     None
//!                 }
//!                 _ => None,
//!             };
//!             if let Some(reply) = reply_msg_request {
//!                 let send_future = rutebot
//!                     .prepare_api_request(reply)
//!                     .send()
//!                     .map(|_| ())
//!                     .map_err(|x| println!("Got error while sending message: {:?}", x));
//!                 hyper::rt::spawn(send_future);
//!             }
//!             Ok(())
//!         });
//!
//!     hyper::rt::run(updates);
//! }
//! ```

/// Telegram bot api responses
pub mod responses;

/// Types for sending requests to telegram bot api
pub mod client;

/// Requests for `client::Rutebot`. Each request struct represent some method telegram bot api
pub mod requests;

/// Errors definitions
pub mod error;

mod updates_pool_stream;

#[cfg(test)]
mod tests {}
