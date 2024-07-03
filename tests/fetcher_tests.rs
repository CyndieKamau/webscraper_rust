// Writing tests for the following cases :
// Fetching a variety of URLs returns a valid response
// Fetching deformed URLs returns an appropriate error
// Test both http, https return appropriate responses


mod helpers;
use helpers::*;

use webscraper_rust::fetcher::fetch;

#[tokio::test]
async fn test_fetch_valid_url() {
    let url = VALID_URL;
    let response = fetch(url).await;
    assert!(response.is_ok())
}

#[tokio::test]
async fn test_fetch_invalid_url() {
    let url = INVALID_URL;
    let response = fetch(url).await;
    assert!(response.is_err())
}

#[tokio::test]
async fn test_fetch_deformed_url() {
    let url = DEFORMED_URL;
    let response = fetch(url).await;
    assert!(response.is_err())
}

#[tokio::test]
async fn test_fetch_empty_url() {
    let url = EMPTY_URL;
    let response = fetch(url).await;
    assert!(response.is_err())
}

#[tokio::test]
async fn test_fetch_http_url() {
    let url = HTTP_URL;
    let response = fetch(url).await;
    assert!(response.is_ok())
}

#[tokio::test]
async fn test_fetch_https_url() {
    let url = HTTPS_URL;
    let response = fetch(url).await;
    assert!(response.is_ok())
}

#[tokio::test]
async fn test_fetch_secure_connection() {
    let url = "https://expired.badssl.com/"; // URL with an expired SSL certificate
    let response = fetch(url).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn test_fetch_non_html() {
    let url = "https://jsonplaceholder.typicode.com/todos/1"; // URL that returns JSON
    let response = fetch(url).await;
    assert!(response.is_ok());
    let binding = response.unwrap();
    let content_type = binding.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.contains("application/json"));
}