use std;
use std::env;
use std::fmt::Debug;

use futures::{Future, IntoFuture};

use rutebot::client::Rutebot;

pub fn create_client() -> Rutebot {
    let token = env::var_os("TEST_TOKEN")
        .expect("Token is missing. You should specify token in TEST_TOKEN env variable");
    let token = token
        .to_string_lossy();

    Rutebot::new(token)
}

pub fn run_one<F>(f: F) -> F::Item
    where
        F: IntoFuture,
        F::Future: Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + Debug + 'static,
{
    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(f.into_future()).unwrap()
}