pub mod responses;
pub mod requests;
pub mod client;

#[cfg(test)]
mod tests {
    use futures::future::Future;

    use crate::requests::get_updates::{AllowedUpdate, GetUpdatesRequest};
    use crate::requests::get_file::GetFileRequest;

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

        let resp_composed = resp.and_then(|_| resp2);

        hyper::rt::run(resp_composed);
        assert_eq!(2 + 2, 4);
    }
}
