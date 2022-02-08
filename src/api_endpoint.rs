use std::{sync::Arc, ops::Deref};
use reqwest::{Method, RequestBuilder, Body};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::client::{BaseClient};

#[derive(Serialize, Deserialize)]
pub struct UsersEndpointListParams {
    start_cursor: Option<String>,
    page_size: i32
}

impl Default for UsersEndpointListParams {
    fn default() -> Self {
        Self {
            start_cursor: None,
            page_size: 100,
        }
    }
}

pub struct UsersEndpoint {
    parent: Arc<BaseClient>
}

impl UsersEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client)
        }
    }

    pub fn list(&self, params: Option<&UsersEndpointListParams>) -> Result<RequestBuilder, reqwest::Error>{
        self.parent.deref().request("users", Method::GET, params, None::<&str>)
    }

    pub fn retrive(&self, user_id: &str) -> Result<RequestBuilder, reqwest::Error> {
        self.parent.deref().request(&format!("users/{}", user_id), Method::GET, None::<&str>, None::<&str>)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SearchEndpointParams {}

impl Into<Body> for SearchEndpointParams {
    fn into(self) -> Body {
        Body::from(serde_json::to_string(&self).unwrap())
    }
}

pub struct SearchEndpoint {
    parent: Arc<BaseClient>
}

impl SearchEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client)
        }
    }

    pub fn search(&self, params: Option<SearchEndpointParams>) -> Result<RequestBuilder, reqwest::Error> {
        self.parent.deref().request("search", Method::POST, None::<&str>, params)
    }
}

pub struct BlocksChildrenEndpoint {}
pub struct BlocksEndpoint {
    pub children: BlocksChildrenEndpoint
}

pub struct DatabasesEndpoint {}

pub struct PagesPropertiesEndpoint {}
pub struct PagesEndpoint {
    pub properties: PagesPropertiesEndpoint
}