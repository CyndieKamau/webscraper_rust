use reqwest::*;
use std::result::Result;

pub async fn fetch(url: &str) -> Result<Response, reqwest::Error> {
    let response = reqwest::get(url).await?;
    Ok(response)
}