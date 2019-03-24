use crate::types::reply_markup::*;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct CallbackData {
    value: String,
}

#[test]
fn test_serialize() {
    let markup: ReplyMarkup = ForceReply::new(true).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, json!({"force_reply":true}));

    let markup: ReplyMarkup = ForceReply::new(true).selective(true).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, json!({"force_reply":true,"selective":true}));

    let markup: ReplyMarkup = ForceReply::new(true).selective(false).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, json!({"force_reply":true,"selective":false}));

    let callback_data = CallbackData {
        value: String::from("cdstruct"),
    };

    let markup: ReplyMarkup = vec![vec![
        InlineKeyboardButton::with_url("url", "tg://user?id=1"),
        InlineKeyboardButton::with_callback_data("cd", "cd"),
        InlineKeyboardButton::with_callback_data_struct("cd", &callback_data).unwrap(),
        InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
        InlineKeyboardButton::with_switch_inline_query_current_chat("siqcc", "siqcc"),
        InlineKeyboardButton::with_callback_game("cg"),
        InlineKeyboardButton::with_pay("pay"),
    ]]
    .into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(
        j,
        json!({
            "inline_keyboard": [
                [
                    {"text":"url","url":"tg://user?id=1"},
                    {"text":"cd","callback_data":"cd"},
                    {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
                    {"text":"siq","switch_inline_query":"siq"},
                    {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
                    {"text":"cg","callback_game":""},
                    {"text":"pay","pay":true}
                ]
            ]
        })
    );

    let row = vec![
        KeyboardButton::new("test"),
        KeyboardButton::new("request contact").request_contact(),
        KeyboardButton::new("request location").request_location(),
    ];
    let serialized_kb = json!({
        "keyboard": [
            [
                {"text":"test"},
                {"text":"request contact","request_contact":true},
                {"text":"request location","request_location":true}
            ]
        ]
    });
    let markup: ReplyMarkup = vec![row.clone()].into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, serialized_kb);

    let markup: ReplyMarkup = ReplyKeyboardMarkup::default().row(row).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, serialized_kb);

    let markup: ReplyMarkup = ReplyKeyboardRemove::default().selective(true).into();
    let j = serde_json::to_value(&markup).unwrap();
    assert_eq!(j, json!({"remove_keyboard":true,"selective":true}));
}
