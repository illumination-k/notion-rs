use reqwest::{Body, Method, RequestBuilder};

use super::build_endpoint;

build_endpoint!(DatabasesEndpoint);

impl DatabasesEndpoint {
    pub fn query<B>(
        &self,
        database_id: &str,
        params: Option<B>,
    ) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        let path = format!("databases/{}", database_id);
        self.parent
            .request(&path, Method::POST, None::<&str>, params)
    }

    pub fn create<B>(&self, params: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.parent.create("databases", params)
    }

    pub fn update<B>(
        &self,
        database_id: &str,
        params: Option<B>,
    ) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        let path = format!("databases/{}", database_id);
        self.parent.update("databases", params)
    }

    pub fn retrive(&self, database_id: &str) -> Result<RequestBuilder, reqwest::Error> {
        let path = format!("databases/{}", database_id);
        self.parent.retrive(&path)
    }
}
