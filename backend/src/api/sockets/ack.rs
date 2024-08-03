/// This modules defines an `Ack` to be returned by
use std::string::String;

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Ack<T> {
    Success { success: T },
    Error { error: String },
}

impl<T> From<Result<T>> for Ack<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(data) => Ack::Success { success: data },
            Err(error) => Ack::Error {
                error: error.to_string(),
            },
        }
    }
}
