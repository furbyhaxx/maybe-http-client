use std::time::Duration;
use reqwest::Method;
#[cfg(feature = "sync")]
use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};
#[cfg(feature = "async")]
use reqwest::{Client, ClientBuilder, RequestBuilder};
use serde_json::Value;
use crate::{Form, Headers, HttpClientError, Query};


#[derive(Debug, Clone)]
pub struct HttpClient {
    /// reqwest needs an instance of its client to perform requests.
    client: Client,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for HttpClient {
    fn default() -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()
            // building with these options cannot fail
            .unwrap();
        Self { client }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for HttpClient {
    fn default() -> Self {
        let client = ClientBuilder::new()
            .build()
            // building with these options cannot fail
            .unwrap();
        Self { client }
    }
}

// #[cfg_attr(feature = "async", maybe_async::maybe_async)]
// #[cfg_attr(feature = "sync", maybe_async::maybe_async)]
#[maybe_async::maybe_async]
impl HttpClient {

    #[maybe_async::maybe_async]
    pub async fn request<D>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        add_data: D,
    ) -> Result<String, HttpClientError>
        where D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut request = self.client.request(method.clone(), url);

        // Setting the headers, if any
        if let Some(headers) = headers {
            // The headers need to be converted into a `reqwest::HeaderMap`,
            // which won't fail as long as its contents are ASCII. This is an
            // internal function, so the condition cannot be broken by the user
            // and will always be true.
            //
            // The content-type header will be set automatically.
            let headers = headers.try_into().unwrap();

            request = request.headers(headers);
        }

        // Configuring the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Finally performing the request and handling the response
        #[cfg(feature = "log")]
        log::debug!("Making request {:?}", request);


        let response = request.send().await?;
        
        // #[cfg(feature = "async")]
        //     let response = request.send().await?;
        // #[cfg(feature = "sync")]
        //     let response = request.send()?;

        // Making sure that the status code is OK
        if response.status().is_success() {
            response.text().await.map_err(Into::into)
            // return response.text().map_err(Into::into);
            // #[cfg(feature = "async")]
            // return response.text().await.map_err(Into::into);
            // #[cfg(feature = "sync")]
            // return response.text().map_err(Into::into);

        } else {
            Err(HttpClientError::StatusCode(response))
        }
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn get(
        &self,
        url: impl AsRef<str>,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, HttpClientError> {
        self.request(reqwest::Method::GET, url.as_ref(), headers, |req| req.query(payload))
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn post(
        &self,
        url: impl AsRef<str>,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, HttpClientError> {
        self.request(reqwest::Method::POST, url.as_ref(), headers, |req| req.json(payload))
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn post_form(
        &self,
        url: impl AsRef<str>,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> Result<String, HttpClientError> {
        self.request(reqwest::Method::POST, url.as_ref(), headers, |req| req.form(payload))
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn put(
        &self,
        url: impl AsRef<str>,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, HttpClientError> {
        self.request(reqwest::Method::PUT, url.as_ref(), headers, |req| req.json(payload))
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn delete(
        &self,
        url: impl AsRef<str>,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, HttpClientError> {
        self.request(reqwest::Method::DELETE, url.as_ref(), headers, |req| req.json(payload))
            .await
    }
}