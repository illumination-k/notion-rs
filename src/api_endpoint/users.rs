use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc};

use crate::client::BaseClient;

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

pub struct UsersEndpoint {
    parent: Arc<BaseClient>,
}

impl UsersEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client),
        }
    }

    pub fn list<Q>(&self, params: Option<&Q>) -> Result<RequestBuilder, reqwest::Error>
    where
        Q: Serialize + ?Sized,
    {
        self.parent
            .deref()
            .request("users", Method::GET, params, None::<&str>)
    }

    pub fn retrive(&self, user_id: &str) -> Result<RequestBuilder, reqwest::Error> {
        self.parent.deref().request(
            &format!("users/{}", user_id),
            Method::GET,
            None::<&str>,
            None::<&str>,
        )
    }

    pub fn me(&self) -> Result<RequestBuilder, reqwest::Error> {
        self.parent
            .deref()
            .request("users/me", Method::GET, None::<&str>, None::<&str>)
    }
}
