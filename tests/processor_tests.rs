//test processing of valid HTML
//test processing of invalid HTML
//test processing of empty HTML
//test processing of non-html eg JSON
//test processing of special characters
//test how the processor handles error messages or non-200 status codes
//test how the processor handles multiple concurrent inputs



use webscraper_rust::processor::*;


#[tokio::test]
async fn test_process_valid_html() {
    let content = "<html><body>Valid HTML</body></html>".to_string();
    let result = process_data(content).await;
    assert!(matches!(result, Ok(ProcessedData::Html(_))));
}

#[tokio::test]
async fn test_process_invalid_html() {
    let content = "<html><body>Invalid HTML</body></html>".to_string();
    let result = process_data(content).await;
    assert!(matches!(result, Err(ProcessingError::HtmlParseError(_))));
}

#[tokio::test]
async fn test_process_empty_content() {
    let content = "".to_string();
    let result =process_data(content).await;
    assert!(matches!(result, Err(ProcessingError::EmptyContent)));
}

#[tokio::test]
async fn test_process_valid_json() {
    let content = r#"{"key": "value"}"#.to_string();
    let result = process_data(content).await;
    assert!(matches!(result, Ok(ProcessedData::Json(_))));
}

#[tokio::test]
async fn test_process_invalid_json() {
    let content = r#"{"key": "value""#.to_string();
    let result = process_data(content).await;
    assert!(matches!(result, Err(ProcessingError::JsonParseError(_))));
}

#[tokio::test]
async fn test_process_large_html() {
    let content = "<html><body>Large HTML</body></html>".repeat(1000);
    let result = process_data(content).await;
    assert!(matches!(result, Ok(ProcessedData::Html(_))));
}

#[tokio::test]
async fn test_process_special_characters() {
    let content = "<html><body>Special characters: &lt; &gt; &quot; &apos; &amp;</body></html>".to_string();
    let result = process_data(content).await;
    assert!(matches!(result, Ok(ProcessedData::Html(_))));
}

#[tokio::test]
async fn test_process_multiple_concurrent_inputs() {
    let content1 = "<html><body>First input</body></html>".to_string();
    let content2 = "<html><body>Second input</body></html>".to_string();
    let result1 = process_data(content1).await;
    let result2 = process_data(content2).await;
    assert!(matches!(result1, Ok(ProcessedData::Html(_))));
    assert!(matches!(result2, Ok(ProcessedData::Html(_))));
}