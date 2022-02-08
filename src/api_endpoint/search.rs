use reqwest::{Body, Method, RequestBuilder};
use std::ops::Deref;

use super::build_endpoint;

build_endpoint!(SearchEndpoint);

impl SearchEndpoint {
    pub fn search<B>(&self, params: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.parent
            .deref()
            .request("search", Method::POST, None::<&str>, params)
    }
}
