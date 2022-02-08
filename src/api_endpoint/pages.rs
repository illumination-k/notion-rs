use std::sync::Arc;

use reqwest::{Body, RequestBuilder};

use crate::client::BaseClient;

use super::build_endpoint;

build_endpoint!(PagesPropertiesEndpoint);

impl PagesPropertiesEndpoint {
    pub fn retrive(
        &self,
        page_id: &str,
        property_id: &str,
    ) -> Result<RequestBuilder, reqwest::Error> {
        let path = format!("pages/{}/properties/{}", page_id, property_id);
        self.parent.retrive(&path)
    }
}

#[derive(Debug, Clone)]
pub struct PagesEndpoint {
    parent: Arc<BaseClient>,
    pub properties: PagesPropertiesEndpoint,
}

impl PagesEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client),
            properties: PagesPropertiesEndpoint::new(client),
        }
    }

    pub fn retrive(&self, page_id: &str) -> Result<RequestBuilder, reqwest::Error> {
        let path = format!("pages/{}", page_id);
        self.parent.retrive(&path)
    }

    pub fn create<B>(&self, params: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.parent.create("pages", params)
    }
}
