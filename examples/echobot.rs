use std::env;

use futures_util::StreamExt;
use rutebot::{
    client::Rutebot,
    requests::SendMessage,
    responses::{Message, Update},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token_env = env::var_os("TELEGRAM_TOKEN")
        .expect("Please specify your bot's token in the TELEGRAM_TOKEN environment variable.");
    let token = token_env.to_string_lossy();

    let rutebot = Rutebot::new(token);
    let mut updates_stream = Box::pin(rutebot.incoming_updates(None, None));
    while let Some(update) = updates_stream.next().await {
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
                let send_future = rutebot.prepare_api_request(request).send();
                tokio::spawn(send_future);
            }
            Err(e) => {
                println!("Got error while getting updates {:?}", e);
            }
            _ => (),
        }
    }
    Ok(())
}
