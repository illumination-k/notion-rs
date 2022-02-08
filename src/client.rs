use reqwest::header::{HeaderValue, USER_AGENT};
use reqwest::{header, Body, Method, RequestBuilder, Url};
use serde::Serialize;
use std::error::Error;

use crate::api_endpoint::{
    BlocksEndpoint, DatabasesEndpoint, PagesEndpoint, SearchEndpoint, UsersEndpoint,
};
use crate::NOTION_ENDPOINT;
use std::sync::Arc;

pub struct NotionClient {
    pub users: UsersEndpoint,
    pub databases: DatabasesEndpoint,
    pub pages: PagesEndpoint,
    pub blocks: BlocksEndpoint,
    search: SearchEndpoint,
}

impl NotionClient {
    pub fn try_new(
        token: &str,
        notion_version: &str,
        user_agent: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let base_client = Arc::new(BaseClient::try_new(token, notion_version, user_agent)?);

        let users = UsersEndpoint::new(&base_client);
        let databases = DatabasesEndpoint::new(&base_client);
        let pages = PagesEndpoint::new(&base_client);
        let blocks = BlocksEndpoint::new(&base_client);
        let search = SearchEndpoint::new(&base_client);
        Ok(Self {
            users,
            databases,
            pages,
            blocks,
            search,
        })
    }

    pub fn search<B>(&self, params: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.search.search(params)
    }
}

#[derive(Debug, Clone)]
pub struct BaseClient {
    url: Url,
    client: reqwest::Client,
}

impl BaseClient {
    pub fn try_new(
        token: &str,
        notion_version: &str,
        user_agent: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(NOTION_ENDPOINT)?;
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(token)?;
        auth_value.set_sensitive(true);

        headers.insert(
            "Notion-Version",
            header::HeaderValue::from_str(notion_version)?,
        );

        if let Some(user_agent) = user_agent {
            headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
        }

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Self { url, client })
    }

    pub fn request<Q, B>(
        &self,
        path: &str,
        method: Method,
        query: Option<&Q>,
        body: Option<B>,
    ) -> Result<RequestBuilder, reqwest::Error>
    where
        Q: Serialize + ?Sized,
        B: Into<Body>,
    {
        let qb = self.client.request(method, self.url.join(path).unwrap());

        let qb = if let Some(query) = query {
            qb.query(query)
        } else {
            qb
        };

        let qb = if let Some(body) = body {
            qb.header(header::CONTENT_TYPE, "application/json")
                .body(body)
        } else {
            qb
        };

        Ok(qb)
    }

    pub fn retrive(&self, path: &str) -> Result<RequestBuilder, reqwest::Error> {
        Ok(self
            .client
            .request(Method::GET, self.url.join(path).unwrap()))
    }

    pub fn create<B>(&self, path: &str, body: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.request(path, Method::POST, None::<&str>, body)
    }

    pub fn update<B>(&self, path: &str, body: Option<B>) -> Result<RequestBuilder, reqwest::Error>
    where
        B: Into<Body>,
    {
        self.request(path, Method::PATCH, None::<&str>, body)
    }
}
