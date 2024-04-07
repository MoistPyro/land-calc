use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{card::CardObject, search::APP_USER_AGENT};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct BulkIndex {
    #[serde(default)]
    object: String,
    #[serde(default)]
    has_more: bool,
    #[serde(default)]
    data: Vec<BulkItem>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
struct BulkItem {
    #[serde(default)]
    object: String,
    #[serde(default)]
    id: Uuid,
    #[serde(default)]
    uri: String, // URI
    #[serde(default)]
    #[serde(rename = "type")]
    data_type: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    download_uri: String, // URI
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    size: i32,
    #[serde(default)]
    content_type: String,
    #[serde(default)]
    content_encoding: String,
}

const TIMEOUT: u64 = 10;

pub async fn get_bulk_from_scryfall() -> Result<Vec<CardObject>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(APP_USER_AGENT));

    let https_client = Client::builder()
        .default_headers(headers)
        .https_only(true)
        .timeout(Duration::from_secs(TIMEOUT))
        .build()?;

    let url = "https://api.scryfall.com/bulk-data";

    let response = https_client
        .get(url)
        .send()
        .await?
        .json::<BulkIndex>()
        .await?;

    let next_item: &BulkItem = response
        .data
        .iter()
        .filter(|o| o.data_type == "oracle_cards")
        .next()
        .unwrap();

    let next_url: String = next_item.download_uri.to_string();

    let cards = https_client
        .get(next_url)
        .send()
        .await?
        .json::<Vec<CardObject>>()
        .await?;

    Ok(cards)
}
