mod slack;

use anyhow::{anyhow, Error, Result};
use reqwest::{
    header, header::AUTHORIZATION, Client as ReqwestClient, Error as ReqwestError,
    Method as HTTPMethod, StatusCode,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error as ReqwestMiddlewareError};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use slack::*;
use std::path::Path;
use strum::EnumIs;
use thiserror::Error as ThisError;
use tokio::time::Duration;
use tracing::{debug, error};
use url::Url;
use ClientError::*;

#[derive(ThisError, Debug, Copy, Clone, Eq, PartialEq, EnumIs, Default)]
pub enum ClientError {
    #[error("Cannot create client")]
    InitializeError,
    #[error("The request is already created.")]
    Conflict, // 409
    #[error("The request isn't found.")]
    NotFound, // 404
    #[error("The requesting API key is not correct.")]
    AuthenticationError, // 401
    #[error("The request was malformed or missing some required parameters")]
    InvalidRequest, // 400
    #[error("Rate limit reached for requests API")]
    RateLimit, // 429
    #[error("Request timed out")]
    Timeout, // 408, 504
    #[error("The API had an error while processing our request")]
    ServiceUnavailableError, // 500, 503
    #[error("JSON Response Parsing was failed")]
    ResponseParseError,
    #[error("Reqwest Middleware error was occurred")]
    ReqwestMiddlewareError,
    #[error("Unknown error was occurred")]
    #[default]
    UnknownError,
}

#[derive(Debug, Clone)]
pub enum AuthorizationHeader {
    Bearer(String),
}

#[derive(Debug, Clone)]
pub struct BaseHeader {
    pub authorization: AuthorizationHeader,
}

impl BaseHeader {
    pub fn new(token: &str) -> Self {
        Self {
            authorization: AuthorizationHeader::Bearer(token.to_string()),
        }
    }

    pub fn to_header(&self) -> header::HeaderMap {
        use AuthorizationHeader::*;
        let mut headers = header::HeaderMap::new();
        match &self.authorization {
            Bearer(token) => {
                let authorization_value =
                    header::HeaderValue::from_str(&format!("Bearer {}", token))
                        .unwrap_or(header::HeaderValue::from_str("").unwrap());
                headers.insert(AUTHORIZATION, authorization_value);
            }
        }
        headers
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    client: ClientWithMiddleware,
    base_url: Url,
}

impl Client {
    pub(super) fn new(
        base_url: Url,
        base_header: BaseHeader,
        timeout: &u8,
        retry: &u8,
    ) -> Result<Self, Error> {
        let reqwest_client = ReqwestClient::builder()
            .default_headers(base_header.to_header())
            .timeout(Duration::from_secs(*timeout as u64))
            .build()
            .map_err(|_| InitializeError)?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(*retry as u32);

        let reqwest_middleware = ClientBuilder::new(reqwest_client)
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let client = Self {
            client: reqwest_middleware,
            base_url,
        };

        Ok(client)
    }

    pub(super) async fn execute<T, U, V>(
        &self,
        method: HTTPMethod,
        endpoint: V,
        body: &T,
    ) -> Result<U, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
        V: AsRef<Path>,
    {
        let endpoint_strs: Vec<&str> = endpoint
            .as_ref()
            .to_str()
            .unwrap_or_default()
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let mut url = self.base_url.to_owned();

        if let Ok(mut url_segment) = url.path_segments_mut() {
            url_segment.pop_if_empty().extend(endpoint_strs);
        }

        debug!("requesting to {}", &url);
        debug!("client is {:?}", &self.client);

        let response = self
            .client
            .request(method, url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("reqwest execute error: {:?}", e);
                ClientError::from(e)
            })?;

        let status = response.status();
        debug!("status is {:?}", &status);

        if status.is_client_error() || status.is_server_error() {
            return Err(anyhow!(ClientError::from(status)));
        }

        response
            .json::<U>()
            .await
            .map_err(|e| {
                error!("response json parsing error: {:?}", e);
                ClientError::from(e)
            })
            .map_err(Error::new)
    }
}

impl From<StatusCode> for ClientError {
    fn from(status: StatusCode) -> Self {
        error!("reqwest returns error http status code: {}", &status);
        match status {
            StatusCode::NOT_FOUND => NotFound,               // 404
            StatusCode::CONFLICT => Conflict,                // 409
            StatusCode::REQUEST_TIMEOUT => Timeout,          // 408
            StatusCode::TOO_MANY_REQUESTS => RateLimit,      // 429
            StatusCode::GATEWAY_TIMEOUT => Timeout,          // 504
            StatusCode::UNAUTHORIZED => AuthenticationError, // 401
            s if s.is_client_error() => InvalidRequest,      // 400
            _ => ServiceUnavailableError,                    // 500
        }
    }
}

impl From<ReqwestError> for ClientError {
    fn from(err: ReqwestError) -> Self {
        error!("reqwest error: {}", &err);
        if err.is_timeout() {
            return Timeout;
        }
        if err.is_connect() {
            return ServiceUnavailableError;
        }
        if err.is_body() {
            return InvalidRequest;
        }
        if err.is_decode() {
            return ResponseParseError;
        }
        UnknownError
    }
}

impl From<ReqwestMiddlewareError> for ClientError {
    fn from(err: ReqwestMiddlewareError) -> Self {
        if let ReqwestMiddlewareError::Reqwest(e) = err {
            return Self::from(e);
        }
        if let ReqwestMiddlewareError::Middleware(e) = err {
            error!("reqwest middleware error: {}", e);
            return ReqwestMiddlewareError;
        }
        UnknownError
    }
}
