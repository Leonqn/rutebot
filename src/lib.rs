//! Crate with bindings to telegram bot api. For usage details see example below and tests in `basic_req_res_tests.rs`.
//! You can find all implemented methods in `requests` module.
//!
//! # Example
//! Simple echo bot. It listens all incoming messages and echos text messages, on other messages it replies with text "I can echo only text message".
//! ```no_run
//! use std::env;
//!
//! use futures::future::Future;
//! use futures::stream::Stream;
//! use rutebot::client::Rutebot;
//! use rutebot::requests::{GetUpdates, SendMessage};
//! use rutebot::responses::Update;
//!
//! fn main() {
//!     let token_env = env::var_os("TELEGRAM_TOKEN")
//!         .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
//!     let token = token_env.to_string_lossy();
//!
//!     let rutebot = Rutebot::new(token);
//!     let get_updates = GetUpdates::new_with_timeout(20);
//!     let updates = rutebot
//!         .incoming_updates(get_updates)
//!         .for_each(move |update| {
//!             let create_reply_request = |update: Update| {
//!                 let message = update.message?;
//!                 let response_message = format!("Hello {}", message.from?.first_name);
//!                 let reply =
//!                     SendMessage::new_reply(message.chat.id, &response_message, message.message_id);
//!                 Some(rutebot.prepare_api_request(reply))
//!             };
//!
//!             if let Some(reply) = create_reply_request(update) {
//!                 tokio::spawn(reply.send().then(|_| Ok(())));
//!             }
//!             Ok(())
//!         })
//!         .then(|_| Ok(()));
//!
//!     tokio::run(updates);
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
