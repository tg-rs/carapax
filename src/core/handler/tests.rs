use std::fmt;

use super::*;

#[test]
fn convert_input() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    assert_eq!(HandlerInput::from(update).update.id, 1);
}

#[derive(Debug)]
struct ExampleError;

impl Error for ExampleError {}

impl fmt::Display for ExampleError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "Example error")
    }
}

#[test]
fn convert() {
    assert!(matches!(().into_result(), Ok(())));
    assert!(matches!(Ok::<(), ExampleError>(()).into_result(), Ok(())));
    assert!(Err::<(), ExampleError>(ExampleError).into_result().is_err());
}
