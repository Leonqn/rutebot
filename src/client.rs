use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

use futures::future::{Future, IntoFuture};
use futures::stream::Stream;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;

use crate::error::Error;
use crate::requests;
use crate::requests::GetUpdates;
use crate::responses::{TgResponse, Update};
use crate::updates_pool_stream::UpdatesPoolStream;

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";
const GET_FILE_URI: &'static str = "https://api.telegram.org/file/bot";

#[derive(Clone)]
struct Inner {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
    token: String,
}

/// Main type for interacting with telegram bot api
#[derive(Clone)]
pub struct Rutebot {
    inner: Arc<Inner>,
}

/// Represents ready request to telegram bot api.
pub struct ApiRequest<TResponse: DeserializeOwned> {
    inner: Arc<Inner>,
    http_request: Result<Request<Body>, Error>,
    _data: PhantomData<TResponse>,
}

impl<TResponse: DeserializeOwned> ApiRequest<TResponse> {
    /// Send request to telegram bot api.
    /// ## Example
    /// ```
    /// # use rutebot::requests::{AllowedUpdate, GetUpdates};
    /// # fn main() {
    /// # let bot = rutebot::client::Rutebot::new("token");
    /// # let allowed_updates = [AllowedUpdate::Message];
    /// # let get_updates = GetUpdates {
    /// #    allowed_updates: Some(&allowed_updates),
    /// #    ..GetUpdates::new()
    /// # };
    /// # let request = bot.prepare_api_request(get_updates);
    /// let future = request.send();
    /// # }
    /// ```
    pub fn send(self) -> impl Future<Item = TResponse, Error = Error> {
        let http_request = self.http_request;
        let bot = self.inner;
        http_request.into_future().and_then(move |http_request| {
            bot.http_client
                .request(http_request)
                .and_then(|r| r.into_body().concat2())
                .then(move |body| {
                    let body_ref = &body.map_err(Error::Hyper)?;
                    let response: TgResponse<TResponse> =
                        serde_json::from_slice(body_ref).map_err(Error::Serde)?;
                    match response {
                        TgResponse {
                            ok: true,
                            result: Some(res),
                            ..
                        } => Ok(res),

                        TgResponse {
                            description,
                            error_code,
                            parameters,
                            ..
                        } => Err(Error::Api {
                            error_code: error_code.unwrap_or(0),
                            description: description.unwrap_or("Unknown error".to_string()),
                            parameters,
                        }),
                    }
                })
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
            inner: Arc::new(Inner { http_client, token }),
        }
    }

    /// Prepare request for sending. Generic method for interaction with telegram bot api.
    ///
    /// Use Request structs from `requests` module for preparing needed api method
    /// ## Example
    /// Prepare request to recieve all unconfirmed messages. After creating request you can send it by method `send()`
    /// ```
    /// # use rutebot::requests::{AllowedUpdate, GetUpdates};
    /// # fn main() {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let allowed_updates = [AllowedUpdate::Message];
    /// let get_updates = GetUpdates {
    ///     allowed_updates: Some(&allowed_updates),
    ///     ..GetUpdates::new()
    /// };
    /// let request = bot.prepare_api_request(get_updates);
    /// # }
    /// ```
    pub fn prepare_api_request<TRequest, TResponse>(
        &self,
        request: TRequest,
    ) -> ApiRequest<TResponse>
    where
        TRequest: requests::Request<ResponseType = TResponse>,
        TResponse: DeserializeOwned,
    {
        let uri = format!("{}{}/{}", BASE_API_URI, self.inner.token, request.method());
        let http_request = request.set_http_request_body(Request::post(uri));

        ApiRequest {
            inner: self.inner.clone(),
            http_request,
            _data: PhantomData,
        }
    }

    /// Download file from telegram. Before downloading you need to prepare file and obtain `file_path`
    /// using `GetFileRequest`, see example below.
    /// ## Example
    /// Download file by it file_id
    /// ```
    /// # use futures::future::Future;
    ///
    /// # use rutebot::requests::GetFile;
    /// # fn main() {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let get_file = GetFile::new("file-id");
    /// let file_fut = bot.prepare_api_request(get_file)
    ///     .send()
    ///     .and_then(move |file| bot.download_file(&file.file_path.as_ref().map_or("ru-RU", String::as_str)));
    ///
    /// # }
    /// ```
    pub fn download_file(&self, file_path: &str) -> impl Future<Item = Vec<u8>, Error = Error> {
        let uri = format!("{}{}/{}", GET_FILE_URI, self.inner.token, file_path)
            .parse()
            .expect("Error has occurred while creating get_file uri");
        self.inner
            .http_client
            .get(uri)
            .map_err(Error::Hyper)
            .and_then(|response| {
                let http_code = response.status();
                response
                    .into_body()
                    .concat2()
                    .map_err(Error::Hyper)
                    .then(move |body| {
                        let body = body?;
                        if http_code.is_success() {
                            Ok(body.to_vec())
                        } else {
                            let response: TgResponse<()> =
                                serde_json::from_slice(&body).map_err(Error::Serde)?;
                            Err(Error::Api {
                                error_code: response.error_code.unwrap_or(0),
                                description: response
                                    .description
                                    .unwrap_or("Unknown error".to_string()),
                                parameters: response.parameters,
                            })
                        }
                    })
            })
    }

    /// Recieve updates using polling.
    /// ## Example
    /// Create future to recieve all incoming messages using long polling with poll timeout 30 seconds
    /// ```
    /// # use futures::future::Future;
    /// # use futures::stream::Stream;
    /// # use rutebot::requests::{AllowedUpdate, GetUpdates};
    /// # fn main() {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let allowed_updates = [AllowedUpdate::Message];
    /// let get_updates = GetUpdates {
    ///     allowed_updates: Some(&allowed_updates),
    ///     timeout: Some(30),
    ///     ..GetUpdates::new()
    /// };
    /// let incoming_updates_future =
    ///     bot.incoming_updates(get_updates)
    ///     .for_each(|update| Ok(()));
    /// # }
    /// ```
    pub fn incoming_updates(
        &self,
        request: GetUpdates,
    ) -> impl Stream<Item = Update, Error = Error> {
        let self_1 = self.clone();
        let allowed_updates = request.allowed_updates.map(|x| x.to_vec());
        let limit = request.limit;
        let timeout = request.timeout;
        let offset = request.offset;
        let send_request = move |x| {
            let request = GetUpdates {
                offset: x,
                limit,
                timeout,
                allowed_updates: allowed_updates.as_ref().map(|x| x.as_slice()),
            };
            self_1.prepare_api_request(request).send()
        };
        let first_request = self.prepare_api_request(request).send();

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
