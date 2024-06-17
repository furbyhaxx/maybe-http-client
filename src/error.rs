#[cfg(feature = "sync")]
use reqwest::blocking::{Client, ClientBuilder, RequestBuilder, Response};
#[cfg(feature = "async")]
use reqwest::{Response};

#[derive(thiserror::Error, Debug)]
pub enum HttpClientError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503.
    #[error("status code {}", Response::status(.0))]
    StatusCode(Response),
}