use std::result::Result;
use std::error::Error;
use std::collections::HashMap;

pub async fn process_data(content: String) -> Result<ProcessedData, ProcessingError> {
    if content.is_empty() {
        return Err(ProcessingError::EmptyContent);
    };

    if is_html(&content) {
        return Ok(ProcessedData::Html(HtmlData { content }));
    } else if is_json(&content) {
        let result: Result<HashMap<String, String>, _> = serde_json::from_str(&content);
        match result {
            Ok(json_data) => Ok(ProcessedData::Json(JsonData { data: json_data })),
            Err(err) => Err(ProcessingError::JsonParseError(Box::new(err))),

        }
    } else {
        return Ok(ProcessedData::Text(content));
    }     
}

fn is_html(content: &str) -> bool {
    content.contains("<html>")
}
fn is_json(content: &str) -> bool {
    content.starts_with("{") && content.ends_with("}")
}

//error types
pub enum ProcessingError {
    EmptyContent,
    MalformedContent,
    UnsupportedContentType,
    JsonParseError(Box<dyn Error>),
    HtmlParseError(Box<dyn Error>),

}

//result of processing the data
pub enum ProcessedData {
    Html(HtmlData),
    Json(JsonData),
    Text(String),
}

pub struct HtmlData {
    pub content: String,
}

pub struct JsonData {
    pub data: HashMap<String, String>,
}
