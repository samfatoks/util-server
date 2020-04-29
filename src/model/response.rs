use crate::error::Error;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub result: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
}

impl Response {
    pub fn new_successful<S: Into<String>>(result: S, execution_time: u128) -> Self {
        Response {
            status: ResponseStatus::Successful,
            result: result.into(),
            error: String::new(),
            data: None,
            execution_time: Some(format!("{} microseconds", execution_time)),
        }
    }

    pub fn new_successful_with_extradata(data: Value, execution_time: u128) -> Self {
        Response {
            status: ResponseStatus::Successful,
            result: String::new(),
            error: String::new(),
            data: Some(data),
            execution_time: Some(format!("{} microseconds", execution_time)),
        }
    }

    pub fn new_failed<S: Into<String>>(error: S) -> Self {
        Response {
            status: ResponseStatus::Failed,
            data: None,
            result: String::new(),
            error: error.into(),
            execution_time: None,
        }
    }
}
impl From<Error> for Response {
    fn from(err: Error) -> Self {
        Response::new_failed(err.to_string())
    }
}
