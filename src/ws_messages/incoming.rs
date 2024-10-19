use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum MessageValues {
    Valid(ParsedMessage, String),
    Invalid(ErrorData),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "name", content = "body")]
pub enum ParsedMessage {
    Backup,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct StructuredMessage {
    data: Option<ParsedMessage>,
    error: Option<ErrorData>,
    unique: String,
}

// TODO
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "error", content = "message")]
pub enum ErrorData {
    Something(String),
}

pub fn to_struct(input: &str) -> Option<MessageValues> {
    let user_serialized = serde_json::from_str::<StructuredMessage>(input);
    if let Ok(message) = user_serialized {
        if let Some(data) = message.error {
            return Some(MessageValues::Invalid(data));
        }
        if let Some(data) = message.data {
            return Some(MessageValues::Valid(data, message.unique));
        }
        None
    } else {
        let error_serialized = serde_json::from_str::<ErrorData>(input);
        error_serialized.map_or_else(
            |_| {
                 tracing::debug!(input);
                tracing::debug!("not a known input message");
                None
            },
            |data| {
                tracing::debug!("Matched error_serialized data");
                Some(MessageValues::Invalid(data))
            },
        )
    }
}

/// message_incoming
///
/// cargo watch -q -c -w src/ -x 'test message_incoming -- --test-threads=1 --nocapture'
#[expect(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_incoming_parse_invalid() {
        let data = "";
        let result = to_struct(data);
        assert!(result.is_none());

        let data = "{}";
        let result = to_struct(data);
        assert!(result.is_none());
    }

    #[test]
    fn message_incoming_parse_backup_data() {
        let data = r#"{
            	"data": {
            		"name" : "backup"
				},
            	"unique": "some_string"
            }"#;
        let result = to_struct(data);
        assert!(result.is_some());
        let result = result.unwrap();
        match result {
            MessageValues::Valid(ParsedMessage::Backup, _unique) => (),
            _ => unreachable!("Shouldn't have matched this"),
        };
    }

    #[test]
    fn message_incoming_invalid_parse_backup_data() {
        // missing unique
        let data = r#"
			 {
				 "data": {
					 "name" : "backup",
				 },
			 }"#;
        let result = to_struct(data);
        assert!(result.is_none());

        // invalid message name
        let data = r#"
			 {
				 "data": {
					 "name" : "invalid_name",
					 },
				"unique": "some_string"
			 }"#;
        let result = to_struct(data);
        assert!(result.is_none());

        // unique not string
        let data = r#"
		 {
			 "data": {
				 "name" : "invalid_name",
				 },
			"unique": 100
		 }"#;
        let result = to_struct(data);
        assert!(result.is_none());
    }
}
