pub mod alert;
pub mod change;

use headers::authorization::{Authorization, Bearer};
use headers::{ContentType, HeaderMapExt};
use hyper::client::connect::HttpConnector;
use hyper::{Body, Method, StatusCode};
use hyper_tls::HttpsConnector;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub const GET: Method = Method::GET;
pub const POST: Method = Method::POST;
pub const PUT: Method = Method::PUT;

#[derive(Debug)]
pub enum ApiType {
    Alert,
    Change,
}

#[derive(Debug)]
pub enum BigPandaError {
    NetworkError(hyper::Error),
    HTTPError(StatusCode),
}

impl Display for BigPandaError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            BigPandaError::NetworkError(ref err) => err.fmt(formatter),
            BigPandaError::HTTPError(ref status_code) => {
                write!(formatter, "Invalid status code: {}", status_code)
            }
        }
    }
}

impl Error for BigPandaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            BigPandaError::NetworkError(ref err) => Some(err),
            _ => None,
        }
    }
}

pub struct Client {
    api_type: ApiType,
    app_key: String,
    auth_token: String,
    auth_header: Authorization<Bearer>,
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    pub fn new(
        api_type: ApiType,
        app_key: &str,
        auth_token: &str
    ) -> Client {
        Client {
            api_type: api_type,
            app_key: app_key.to_string(),
            auth_token: auth_token.to_string(),
            auth_header: Authorization::bearer(auth_token).unwrap(),
            http_client: hyper::Client::builder().build(HttpsConnector::new()),
        }
    }

    async fn send_request(
        &self,
        method: Method,
        url: &str,
        body: Body,
    ) -> Result<(), BigPandaError> {
        let mut request = hyper::Request::builder()
            .method(method)
            .uri(&*url)
            .body(body)
            .unwrap();

        request.headers_mut().typed_insert(ContentType::json());
        request.headers_mut().typed_insert(self.auth_header.clone());

        // BigPanda's Change API is unique and requires an app key header
        if matches!(self.api_type, ApiType::Change) {
            request.headers_mut()
                .append("x-bp-app-key", self.app_key.parse().unwrap());
        }

        let response = self
            .http_client
            .request(request)
            .await
            .map_err(BigPandaError::NetworkError)?;

        match response.status() {
            StatusCode::CREATED | StatusCode::OK | StatusCode::NO_CONTENT => {}
            error => return Err(BigPandaError::HTTPError(error)),
        };

        Ok(())
    }
}
