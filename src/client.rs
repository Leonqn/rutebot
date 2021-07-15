use std::sync::Arc;

use crate::{
    error::Error,
    requests,
    requests::{GetUpdates, UpdateKind},
    responses::{TgResponse, Update},
};
use bytes::Buf;
use futures_util::{stream::Stream, StreamExt, TryStreamExt};
use hyper::{client::HttpConnector, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;

use crate::responses::ResponseParameters;
use std::io::Read;
use std::marker::PhantomData;
use std::time::Duration;

const BASE_API_URI: &str = "https://api.telegram.org/bot";
const GET_FILE_URI: &str = "https://api.telegram.org/file/bot";

struct Inner {
    http_client: Client<HttpsConnector<HttpConnector>>,
    token: String,
}

/// Main type for interacting with telegram bot api
#[derive(Clone)]
pub struct Rutebot {
    inner: Arc<Inner>,
}

/// Represents ready request to telegram bot api.
#[must_use = "ApiRequest should be sent"]
pub struct ApiRequest<TResponse: DeserializeOwned> {
    inner: Arc<Inner>,
    http_request: Result<Request<Body>, Error>,
    _data: PhantomData<TResponse>,
}

impl<TResponse: DeserializeOwned> ApiRequest<TResponse> {
    /// Send request to telegram bot api.
    /// ## Example
    /// ```
    /// # use rutebot::requests::{UpdateKind, GetUpdates};
    /// # let bot = rutebot::client::Rutebot::new("token");
    /// # let allowed_updates = [UpdateKind::Message];
    /// # let get_updates = GetUpdates {
    /// #    allowed_updates: Some(&allowed_updates),
    /// #    ..GetUpdates::new()
    /// # };
    /// # let request = bot.prepare_api_request(get_updates);
    /// let future = request.send();
    /// ```
    pub async fn send(self) -> Result<TResponse, Error> {
        let http_request = self.http_request;

        let response = self
            .inner
            .http_client
            .request(http_request?)
            .await
            .map_err(Error::Hyper)?;

        let body = hyper::body::aggregate(response)
            .await
            .map_err(Error::Hyper)?;
        let response: TgResponse<TResponse> =
            serde_json::from_reader(body.reader()).map_err(Error::Serde)?;

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
                description: description.unwrap_or_else(|| "Unknown error".to_string()),
                parameters,
            }),
        }
    }
}

impl Rutebot {
    /// Create telegram bot api client
    pub fn new<S: Into<String>>(token: S) -> Self {
        let http_client = Client::builder().build::<_, Body>(HttpsConnector::new());
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
    /// # use rutebot::requests::{UpdateKind, GetUpdates};
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let allowed_updates = [UpdateKind::Message];
    /// let get_updates = GetUpdates {
    ///     allowed_updates: Some(&allowed_updates),
    ///     ..GetUpdates::new()
    /// };
    /// let response = bot.prepare_api_request(get_updates);
    /// ```
    pub fn prepare_api_request<TRequest, TResponse>(
        &self,
        request: TRequest,
    ) -> ApiRequest<TResponse>
    where
        TRequest: requests::Request<ResponseType = TResponse>,
        TResponse: DeserializeOwned + 'static,
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
    /// Download file by its file_id
    /// ```
    /// # use rutebot::requests::GetFile;
    /// # async {
    /// let bot = rutebot::client::Rutebot::new("token");
    /// let get_file = GetFile::new("file-id");
    /// let file_handle = bot.prepare_api_request(get_file).send().await.unwrap();
    /// let file_bytes = bot.download_file(file_handle.file_path.as_ref().map_or("ru-RU", String::as_str)).await.unwrap();
    /// # };
    /// ```
    pub async fn download_file(&self, file_path: &str) -> Result<Vec<u8>, Error> {
        let uri = format!("{}{}/{}", GET_FILE_URI, self.inner.token, file_path)
            .parse()
            .expect("Error has occurred while creating get_file uri");
        let response = self
            .inner
            .http_client
            .get(uri)
            .await
            .map_err(Error::Hyper)?;
        let http_code = response.status();
        let body = hyper::body::aggregate(response)
            .await
            .map_err(Error::Hyper)?;

        if http_code.is_success() {
            let mut response_bytes = Vec::with_capacity(body.remaining());
            let mut reader = body.reader();
            reader.read_to_end(&mut response_bytes).map_err(Error::IO)?;
            Ok(response_bytes)
        } else {
            let response: TgResponse<()> =
                serde_json::from_reader(body.reader()).map_err(Error::Serde)?;
            Err(Error::Api {
                error_code: response.error_code.unwrap_or(0),
                description: response
                    .description
                    .unwrap_or_else(|| "Unknown error".to_string()),
                parameters: response.parameters,
            })
        }
    }

    pub fn incoming_updates(
        &self,
        start_offset: Option<i64>,
        updates_filter: Option<Vec<UpdateKind>>,
    ) -> impl Stream<Item = Result<Update, Error>> {
        let api = self.clone();
        futures_util::stream::unfold(
            (start_offset, updates_filter, api),
            |(offset, updates_filter, api)| async move {
                let request = GetUpdates {
                    offset,
                    limit: None,
                    timeout: Some(10),
                    allowed_updates: updates_filter.as_deref(),
                };
                let response = api.prepare_api_request(request).send().await;
                let new_offset = match &response {
                    Ok(updates) => updates
                        .iter()
                        .map(|update| update.update_id)
                        .max()
                        .map(|max_update_id| max_update_id + 1),
                    Err(Error::Api {
                        error_code: 429,
                        parameters:
                            Some(ResponseParameters {
                                retry_after: Some(retry_after),
                                ..
                            }),
                        ..
                    }) => {
                        tokio::time::sleep(Duration::from_secs(*retry_after as u64)).await;
                        offset
                    }
                    Err(Error::Serde(_)) => Some(-1),
                    _ => offset,
                };

                Some((response, (new_offset, updates_filter, api)))
            },
        )
        .map_ok(|updates| futures_util::stream::iter(updates).map(Ok))
        .try_flatten()
    }
}
