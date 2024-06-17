mod error;
mod client;

use std::collections::HashMap;
#[cfg(all(feature = "async", feature = "sync"))]
compile_error!(
    "`async` and `sync` features cannot both be enabled at \
    the same time, if you want to use `blocking` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!(
    "You have to enable at least one of the \
    `async` or `sync` features."
);

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;
pub type Form = HashMap<String, String>;

pub use error::HttpClientError;

pub use client::HttpClient;