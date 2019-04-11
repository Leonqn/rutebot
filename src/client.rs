use std::marker::PhantomData;
use std::sync::Arc;

use futures::future::Future;
use futures::stream::Stream;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json;

use crate::requests;
use crate::responses::TgResponse;

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";
const GET_FILE_URI: &'static str = "https://api.telegram.org/file/bot";

#[derive(Clone)]
pub struct Rutebot {
    inner: Arc<Inner>,
}

#[derive(Clone)]
struct Inner {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
    token: String,
}

#[derive(Clone)]
pub struct ApiRequest<TResponse: DeserializeOwned> {
    inner: Arc<Inner>,
    request_body: Vec<u8>,
    method: &'static str,
    _data: PhantomData<TResponse>,
}

impl<TResponse: DeserializeOwned> ApiRequest<TResponse> {
    pub fn send(self) -> impl Future<Item=TResponse, Error=()> {
        let uri = format!("{}{}/{}", BASE_API_URI, self.inner.token, self.method);
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(self.request_body))
                .expect("While creating request an error has occurred");

        self.inner.http_client.request(request)
            .and_then(|r| r.into_body().concat2())
            .then(move |body| {
                let body_ref = &body.map_err(|_| ())?;
                let response: TgResponse<TResponse> = serde_json::from_slice(body_ref).map_err(|_| ())?;
                match response {
                    TgResponse { ok: true, result: Some(res), .. } =>
                        Ok(res),

                    _ =>
                        Err(())
                }
            })
    }
}

impl Rutebot {
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
}