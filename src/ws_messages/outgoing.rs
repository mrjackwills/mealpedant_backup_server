use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

/// Responses, either sent as is, or nested in StructuredResponse below
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "name", content = "body")]
pub enum Response {
    BackupData(BackupData),
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupData {
    pub file_name: String,
    pub file_as_b64: String,
}

/// These get sent to the websocket server when in structured_data mode,
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructuredResponse {
    data: Option<Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Response>,
    unique: String,
}

impl StructuredResponse {
    /// Convert a ResponseMessage into a Tokio message of StructureResponse
    pub fn data(data: Response, unique: String) -> Message {
        let x = Self {
            data: Some(data),
            error: None,
            unique,
        };
        Message::Text(serde_json::to_string(&x).unwrap_or_default())
    }

    /// Convert a ErrorResponse into a Tokio message of StructureResponse
    pub fn _error(data: Response, unique: String) -> Message {
        let x = Self {
            error: Some(data),
            data: None,
            unique,
        };
        Message::Text(serde_json::to_string(&x).unwrap_or_default())
    }
}
