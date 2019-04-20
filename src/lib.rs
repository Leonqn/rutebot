//! Crate with bindings to telegram bot api. For details see example below.
//! You can find all implemented methods in `requests` module.
//!
//! # Example
//! Simple echo bot. It listens all incoming messages and echos text messages, on other messages it replies with text "I can echo only text message".
//! ```no_run
//! use hyper::rt::{Future, Stream};
//!
//! use rutebot::client::Rutebot;
//! use rutebot::requests::get_updates::{GetUpdatesRequest, AllowedUpdate};
//! use rutebot::requests::send_message::send_text::SendTextRequest;
//! use rutebot::responses::{Message, Update};
//!
//! fn main() {
//!    let rutebot = Rutebot::new("token");
//!    let allowed_updates = [AllowedUpdate::Message];
//!    let get_updates =
//!        GetUpdatesRequest {
//!            timeout: Some(20),
//!            allowed_updates: Some(&allowed_updates),
//!            ..GetUpdatesRequest::new()
//!        };
//!    let updates = rutebot.incoming_updates(get_updates)
//!        .then(Ok)
//!        .for_each(move |x| {
//!            let reply_msg_request =
//!                match x {
//!                    Ok(Update { message: Some(Message { message_id, ref chat, text: Some(ref text), .. }), .. }) => {
//!                        let request =
//!                            SendTextRequest::new_reply(chat.id, text, message_id);
//!                        Some(request)
//!                    }
//!                    Ok(Update { message: Some(Message { message_id, ref chat, .. }), .. }) => {
//!                        let request = SendTextRequest::new_reply(chat.id, "I can echo only text message", message_id);
//!                        Some(request)
//!                    }
//!                    Err(e) => {
//!                       println!("Got error while getting updates {:?}", e);
//!                       None
//!                    }
//!                    _ => None
//!                };
//!            if let Some(reply) = reply_msg_request {
//!                let send_future = rutebot.prepare_api_request(reply)
//!                    .send()
//!                    .map(|_| ())
//!                    .map_err(|x| println!("Got error while sending message: {:?}", x));
//!                hyper::rt::spawn(send_future);
//!            }
//!            Ok(())
//!        });
//!
//!    hyper::rt::run(updates);
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

mod updates_poll_stream;

#[cfg(test)]
mod tests {}
