use crate::types::Message;

use super::*;

fn create_command(command: &str) -> Command {
    let len = command.len();
    let message: Message = serde_json::from_value(serde_json::json!(
        {
            "message_id": 1111,
            "date": 0,
            "from": {"id": 1, "is_bot": false, "first_name": "test"},
            "chat": {"id": 1, "type": "private", "first_name": "test"},
            "text": command,
            "entities": [
                {"type": "bot_command", "offset": 0, "length": len}
            ]
        }
    ))
    .unwrap();
    Command::try_from(message).unwrap()
}

#[tokio::test]
async fn command_predicate() {
    let handler = CommandPredicate::new("/start");
    assert!(handler.handle(create_command("/start")).await,);
    assert!(!handler.handle(create_command("/unexpected")).await);
}
