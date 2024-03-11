use crate::card::{ResponseList, SearchResult};
use futures::{stream, StreamExt};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use std::time::Duration;

const CONCURRENT_REQUESTS: usize = 9;
const TIMEOUT: u64 = 3;
const SCRYFALL_URL: &str = "https://api.scryfall.com/cards/search";
pub(crate) const APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const CONNECTION: &str = "keep-alive";

pub async fn get_from_scryfall(
    list: Vec<(u32, String)>,
) -> Result<Vec<(u32, SearchResult)>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(APP_USER_AGENT));
    headers.insert("Connection", HeaderValue::from_static(CONNECTION));

    let https_client = Client::builder()
        .default_headers(headers)
        .https_only(true)
        .timeout(Duration::from_secs(TIMEOUT))
        .build()?;

    let responses = stream::iter(list)
        .map(|(amount, query)| {
            let client = &https_client;
            async move {
                (
                    query.to_string(),
                    scryfall_search(amount, client, &query).await,
                )
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    Ok(responses
        .map(|(q, item)| {
            let resolved = item.unwrap();
            (resolved.0, resolved.1.card_or(q))
        })
        .collect::<Vec<(u32, SearchResult)>>()
        .await)
}

async fn scryfall_search(
    amount: u32,
    client: &Client,
    query: &str,
) -> Result<(u32, ResponseList), reqwest::Error> {
    let response: Response = client
        .get(SCRYFALL_URL)
        .query(&[("q", query)])
        .send()
        .await?;

    return Ok((amount, response.json::<ResponseList>().await?));
}
