//! Crate with bindings to telegram bot api.s


/// Telegram bot api responses
pub mod responses;
/// Requests that you can send to telegram bot api
pub mod requests;
/// Client library for sending requests
pub mod client;
/// Possible errors definition
pub mod error;
mod updates_poll_stream;

#[cfg(test)]
mod tests {
    use futures::future::Future;
    use futures::stream::Stream;

    use crate::requests::get_file::GetFileRequest;
    use crate::requests::get_updates::{AllowedUpdate, GetUpdatesRequest};

    #[test]
    fn it_works() {
        let bot = crate::client::Rutebot::new("");
        let updates = vec![AllowedUpdate::Message];
        let get_updates = GetUpdatesRequest {
            allowed_updates: Some(&updates),
            ..GetUpdatesRequest::new()
        };
        let file_id = String::from("asdasd");
        let get_file = GetFileRequest::new(&file_id);

        let resp = bot
            .create_api_request(&get_updates)
            .send()
            .map(|_| ());

        let resp2 = bot.create_api_request(&get_file).send().map(|_| ());

        let resp_composed = resp.and_then(|_| resp2).map_err(|_| ());

        hyper::rt::run(resp_composed);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        let bot = crate::client::Rutebot::new("");
        let updates = [AllowedUpdate::Message];
        let get_updates = GetUpdatesRequest {
            allowed_updates: Some(&updates),
            ..GetUpdatesRequest::new()
        };
        let file_id = String::from("asdasd");
        let get_file = GetFileRequest::new(&file_id);

        let resp = bot
            .incoming_updates(&get_updates)
            .for_each(|_| Ok(()))
            .map_err(|_| ());

        hyper::rt::run(resp);
        assert_eq!(2 + 2, 4);
    }
}
