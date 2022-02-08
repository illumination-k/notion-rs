use std::env;
use std::error::Error;

pub const NOTION_ENDPOINT: &'static str = "https://api.notion.com/v1/";
pub const NOTION_VERSION: &'static str = "2021-08-16";

mod client;
mod api_endpoint;
use crate::client::{NotionClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = NotionClient::try_new(&env::var("NOTION_TOKEN")?, NOTION_VERSION, None)?;
    dbg!(&client.users.list(None)?.send().await?.text().await?);
    dbg!(&client.search(None)?.send().await?.text().await?);
    Ok(())
}
