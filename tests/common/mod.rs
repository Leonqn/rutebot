use std::{self, env};

use rutebot::client::Rutebot;
use str;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn create_client() -> Rutebot {
    let token = env::var_os("TEST_TOKEN")
        .expect("Please specify a token in the TEST_TOKEN environment variable.\nThat bot will be used for sending the test messages.");
    let token = token.to_string_lossy();

    Rutebot::new(token)
}

pub fn get_chat_id() -> i64 {
    let chat_id = env::var_os("TEST_CHAT_ID").expect(
        "Please specify a supergroup's chat id in the TEST_CHAT_ID environment variable.\nThat group will be used to send test messages \
         to. Ensure that the bot used for testing is an admin.",
    );
    let chat_id = chat_id.to_string_lossy();

    str::parse(&chat_id).unwrap()
}

pub fn get_user_id() -> i64 {
    let user_id = env::var_os("TEST_USER_ID").expect(
        "Please specify a user id in the TEST_USER_ID environment variable.\nThat user will be used for test fetching of a profile \
         picture. You can use a bot's id.",
    );
    let user_id = user_id.to_string_lossy();

    str::parse(&user_id).unwrap()
}
