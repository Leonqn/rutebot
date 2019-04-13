use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::Future;
use futures::stream::Stream;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json;
use tokio::timer::Delay;

use crate::error;
use crate::error::Error;
use crate::requests;
use crate::requests::get_updates::GetUpdatesRequest;
use crate::responses::{TgResponse, Update};
use crate::updates_poll_stream::UpdatesPoolStream;

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";
const GET_FILE_URI: &'static str = "https://api.telegram.org/file/bot";

#[derive(Clone)]
struct Inner {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
    token: String,
}

/// Main type for sending requests to telegram bot api
#[derive(Clone)]
pub struct Rutebot {
    inner: Arc<Inner>,
}

/// Represents ready request to telegram bot api.
#[derive(Clone)]
pub struct ApiRequest<TResponse: DeserializeOwned> {
    inner: Arc<Inner>,
    request_body: Vec<u8>,
    method: &'static str,
    _data: PhantomData<TResponse>,
}

impl<TResponse: DeserializeOwned> ApiRequest<TResponse> {
    /// Send request to telegram bot api
    /// ```
    /// # use rutebot::requests::get_updates::{AllowedUpdate, GetUpdatesRequest};
    /// # fn main() {
    /// # let bot = rutebot::client::Rutebot::new("token");
    /// # let allowed_updates = [AllowedUpdate::Message];
    /// # let get_updates = GetUpdatesRequest {
    /// #    allowed_updates: Some(&allowed_updates),
    /// #    ..GetUpdatesRequest::new()
    /// # };
    /// # let request = bot.create_api_request(&get_updates);
    /// let future = request.send();
    /// # }
    /// ```
    pub fn send(self) -> impl Future<Item=TResponse, Error=Error> {
        let uri = format!("{}{}/{}", BASE_API_URI, self.inner.token, self.method);
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(self.request_body))
                .expect("While creating request an error has occurred");

        self.inner.http_client.request(request)
            .and_then(|r| r.into_body().concat2())
            .then(move |body| {
                let body_ref = &body.map_err(Error::Hyper)?;
                let response: TgResponse<TResponse> = serde_json::from_slice(body_ref).map_err(Error::Serde)?;
                match response {
                    TgResponse { ok: true, result: Some(res), .. } =>
                        Ok(res),

                    TgResponse { description, error_code, parameters, .. } =>
                        Err(Error::Api {
                            error_code: error_code.unwrap_or(0),
                            description: description.unwrap_or("Unknown error".to_string()),
                            parameters,
                        }),
                }
            })
    }
}

impl Rutebot {
    /// Create telegram bot api client
    pub fn new<S: Into<String>>(token: S) -> Self {
        let http_client = Client::builder()
            .build::<_, Body>(HttpsConnector::new(1).expect("TLS initialization failed"));
        let token = token.into();

        Rutebot {
            inner: Arc::new(
                Inner {
                    http_client,
                    token,
                })
        }
    }

    /// Create api request that you can send
    ///
    /// Create request to recieve all unconfirmed messages. After creating request you can send it by method `send()`
    /// ```
    /// # use rutebot::requests::get_updates::{AllowedUpdate, GetUpdatesRequest};
    /// # fn main() {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let allowed_updates = [AllowedUpdate::Message];
    /// let get_updates = GetUpdatesRequest {
    ///     allowed_updates: Some(&allowed_updates),
    ///     ..GetUpdatesRequest::new()
    /// };
    /// let request = bot.create_api_request(&get_updates);
    /// # }
    /// ```
    pub fn create_api_request<TRequest, TResponse>(&self, request: &TRequest) -> ApiRequest<TResponse>
        where TRequest: requests::Request<TResponse>,
              TResponse: DeserializeOwned,
    {
        ApiRequest {
            inner: self.inner.clone(),
            request_body: serde_json::to_vec(request).expect("Error while serializing request"),
            method: request.method(),
            _data: PhantomData,
        }
    }

    /// Recieve updates using polling.
    ///
    /// Create future to recieve all incoming messages using long polling with poll timeout 30 seconds
    /// ```
    /// # use futures::future::Future;
    /// # use futures::stream::Stream;
    /// # use rutebot::requests::get_updates::{AllowedUpdate, GetUpdatesRequest};
    /// # fn main() {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let allowed_updates = [AllowedUpdate::Message];
    /// let get_updates = GetUpdatesRequest {
    ///     allowed_updates: Some(&allowed_updates),
    ///     timeout: Some(30),
    ///     ..GetUpdatesRequest::new()
    /// };
    /// let incoming_updates_future =
    ///     bot.incoming_updates(&get_updates)
    ///     .for_each(|update| Ok(()));
    /// # }
    /// ```
    pub fn incoming_updates<'a>(&self, request: &GetUpdatesRequest<'a>) -> impl Stream<Item=Update, Error=Error> {
        let self_1 = self.clone();
        let allowed_updates = request.allowed_updates.map(|x| x.to_vec());
        let limit = request.limit;
        let timeout = request.timeout;
        let offset = request.offset;
        let send_request = move |x| {
            let request = GetUpdatesRequest {
                offset: x,
                limit,
                timeout,
                allowed_updates: allowed_updates.as_ref().map(|x| x.as_slice()),
            };
            self_1.create_api_request(&request).send()
        };
        let first_request = self.create_api_request(request).send();

        UpdatesPoolStream {
            send_request,
            buffer: VecDeque::new(),
            executing_request: first_request,
            is_canceled: false,
            last_id: offset,
            has_error: false,
        }
    }
}