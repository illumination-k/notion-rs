use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};

use super::build_endpoint;

#[derive(Serialize, Deserialize)]
pub struct ListParams {
    start_cursor: Option<String>,
    page_size: i32,
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            start_cursor: None,
            page_size: 100,
        }
    }
}

build_endpoint!(UsersEndpoint);

impl UsersEndpoint {
    pub fn list<Q>(&self, params: Option<&Q>) -> Result<RequestBuilder, reqwest::Error>
    where
        Q: Serialize + ?Sized,
    {
        self.parent
            .request("users", Method::GET, params, None::<&str>)
    }

    pub fn retrive(&self, user_id: &str) -> Result<RequestBuilder, reqwest::Error> {
        self.parent.retrive(&format!("users/{}", user_id))
    }

    pub fn me(&self) -> Result<RequestBuilder, reqwest::Error> {
        self.parent
            .request("users/me", Method::GET, None::<&str>, None::<&str>)
    }
}
