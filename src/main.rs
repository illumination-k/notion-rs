use std::env;
use std::error::Error;

pub const NOTION_ENDPOINT: &str = "https://api.notion.com/v1/";
pub const NOTION_VERSION: &str = "2021-08-16";

mod api_endpoint;
mod client;
use crate::api_endpoint::params::search::{Filter, FilterValue, Params};

use serde_json::Value;

use crate::client::NotionClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = NotionClient::try_new(&env::var("NOTION_TOKEN")?, NOTION_VERSION, None)?;
    dbg!(
        &client
            .users
            .list(None::<&str>)?
            .send()
            .await?
            .text()
            .await?
    );
    let search_params = Params::new().with_filter(Filter::new(FilterValue::Database));
    dbg!(serde_json::from_str::<Value>(
        &client
            .search(Some(search_params))?
            .send()
            .await?
            .text()
            .await?
    )
    .unwrap());
    // dbg!(serde_json::to_string(&search_params));

    dbg!(
        client
            .databases
            .retrive("5a847334-fcf0-42cd-a006-1f38ab0489a2")?
            .send()
            .await?
            .text()
            .await?
    );
    Ok(())
}
