use crate::types::reply_markup::*;

#[test]
fn test_serialize() {
    let markup: ReplyMarkup = ForceReply::new(true).into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, r#"{"force_reply":true}"#);

    let markup: ReplyMarkup = ForceReply::new(true).with_selective(true).into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, r#"{"force_reply":true,"selective":true}"#);

    let markup: ReplyMarkup = ForceReply::new(true).with_selective(false).into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, r#"{"force_reply":true,"selective":false}"#);

    let markup: ReplyMarkup = InlineKeyboardMarkup::with_keyboard(vec![vec![
        InlineKeyboardButton::new("test"),
        InlineKeyboardButton::with_url("url", "tg://user?id=1"),
        InlineKeyboardButton::with_callback_data("cd", "cd"),
        InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
        InlineKeyboardButton::with_switch_inline_query_current_chat("siqcc", "siqcc"),
        InlineKeyboardButton::with_callback_game("cg"),
        InlineKeyboardButton::with_pay("pay"),
    ]])
    .into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, r#"{"inline_keyboard":[[{"text":"test"},{"text":"url","url":"tg://user?id=1"},{"text":"cd","callback_data":"cd"},{"text":"siq","switch_inline_query":"siq"},{"text":"siqcc","switch_inline_query_current_chat":"siqcc"},{"text":"cg","callback_game":""},{"text":"pay","pay":true}]]}"#);

    let row = vec![
        KeyboardButton::new("test"),
        KeyboardButton::new("request contact").with_request_contact(),
        KeyboardButton::new("request location").with_request_location(),
    ];
    let serialized_kb = r#"{"keyboard":[[{"text":"test"},{"text":"request contact","request_contact":true},{"text":"request location","request_location":true}]]}"#;
    let markup: ReplyMarkup = ReplyKeyboardMarkup::with_keyboard(vec![row.clone()]).into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, serialized_kb);

    let mut kb = ReplyKeyboardMarkup::new();
    kb.add_row(row);
    let markup: ReplyMarkup = kb.into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, serialized_kb);

    let markup: ReplyMarkup = ReplyKeyboardRemove::new().with_selective(true).into();
    let j = serde_json::to_string(&markup).unwrap();
    assert_eq!(j, r#"{"remove_keyboard":true,"selective":true}"#);
}
