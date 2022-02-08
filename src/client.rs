use reqwest::header::{USER_AGENT, HeaderValue};
use reqwest::{header, Body, Url, RequestBuilder};
use serde::Serialize;
use std::error::Error;

use std::sync::Arc;
use crate::NOTION_ENDPOINT;
use crate::api_endpoint::{UsersEndpoint, SearchEndpoint, SearchEndpointParams};

pub struct NotionClient {
    pub users: UsersEndpoint,
    search: SearchEndpoint
}

impl NotionClient {
    pub fn try_new(token: &str, notion_version: &str, user_agent: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let base_client = Arc::new(BaseClient::try_new(token, notion_version, user_agent)?);

        let users = UsersEndpoint::new(&base_client);
        let search = SearchEndpoint::new(&base_client);
        Ok(Self {
            users,
            search
        })
    }

    pub fn search(&self, params: Option<SearchEndpointParams>) -> Result<RequestBuilder, reqwest::Error> {
        self.search.search(params)
    }
}


#[derive(Debug, Clone)]
pub struct BaseClient {
    url: Url,
    client: reqwest::Client,
}

impl BaseClient {
    pub fn try_new(token: &str, notion_version: &str, user_agent: Option<&str>) -> Result<Self, Box<dyn Error>> {
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
        method: reqwest::Method,
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
            qb.body(body)
        } else {
            qb
        };

        Ok(qb)
    }
}