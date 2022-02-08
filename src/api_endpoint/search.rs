use reqwest::{Body, Method, RequestBuilder};
use std::{ops::Deref, sync::Arc};

use crate::client::BaseClient;

pub struct SearchEndpoint {
    parent: Arc<BaseClient>,
}

impl SearchEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client),
        }
    }

    pub fn search<B>(&self, params: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.parent
            .deref()
            .request("search", Method::POST, None::<&str>, params)
    }
}
