use crate::types::chat::Chat;

use serde_json::json;

#[test]
fn test_deserialize_channel() {
    let input = json!({
        "id": 1,
        "type": "channel",
        "title": "channeltitle",
        "username": "channelusername",
        "photo": {
            "small_file_id": "smallfileid",
            "big_file_id": "bigfileid"
        },
        "description": "channeldescription",
        "invite_link": "channelinvitelink",
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "text": "test"
        }
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Channel(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("channeltitle"));
        assert_eq!(chat.username, Some(String::from("channelusername")));
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, String::from("smallfileid"));
        assert_eq!(photo.big_file_id, String::from("bigfileid"));
        assert_eq!(chat.description, Some(String::from("channeldescription")));
        assert_eq!(chat.invite_link, Some(String::from("channelinvitelink")));
        assert!(chat.pinned_message.is_some());
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
    let input = json!({
        "id": 1,
        "type": "channel",
        "title": "channeltitle"
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Channel(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("channeltitle"));
        assert_eq!(chat.username.is_none(), true);
        assert_eq!(chat.photo.is_none(), true);
        assert_eq!(chat.description.is_none(), true);
        assert_eq!(chat.invite_link.is_none(), true);
        assert_eq!(chat.pinned_message.is_none(), true);
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

#[test]
fn test_deserialize_group() {
    let input = json!({
        "id": 1,
        "type": "group",
        "title": "grouptitle",
        "all_members_are_administrators": true,
        "photo": {
            "small_file_id": "smallfileid",
            "big_file_id": "bigfileid"
        },
        "invite_link": "groupinvitelink",
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle",
                "all_members_are_administrators": true
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "user"
            },
            "text": "test"
        }
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Group(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("grouptitle"));
        assert_eq!(chat.all_members_are_administrators, true);
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, String::from("smallfileid"));
        assert_eq!(photo.big_file_id, String::from("bigfileid"));
        assert_eq!(chat.invite_link, Some(String::from("groupinvitelink")));
        assert!(chat.pinned_message.is_some());
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
    let input = json!({
        "id": 1,
        "type": "group",
        "title": "grouptitle",
        "all_members_are_administrators": false
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Group(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("grouptitle"));
        assert_eq!(chat.all_members_are_administrators, false);
        assert_eq!(chat.photo.is_none(), true);
        assert_eq!(chat.invite_link.is_none(), true);
        assert_eq!(chat.pinned_message.is_none(), true);
    } else {
        panic!("Unexpected chat: {:?}", chat);
    }
}

#[test]
fn test_deserialize_private() {
    let input = json!({
        "id": 1,
        "type": "private",
        "username": "testusername",
        "first_name": "testfirstname",
        "last_name": "testlastname",
        "photo": {
            "small_file_id": "smallfileid",
            "big_file_id": "bigfileid"
        }
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Private(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.username, Some(String::from("testusername")));
        assert_eq!(chat.first_name, String::from("testfirstname"));
        assert_eq!(chat.last_name, Some(String::from("testlastname")));
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.big_file_id, "bigfileid");
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }

    let input = json!({
        "id": 1,
        "type": "private",
        "first_name": "testfirstname"
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Private(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.username.is_none(), true);
        assert_eq!(chat.first_name, String::from("testfirstname"));
        assert_eq!(chat.last_name.is_none(), true);
        assert_eq!(chat.photo.is_none(), true);
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}

#[test]
fn test_deserialize_supergroup() {
    let input = json!({
        "id": 1,
        "type": "supergroup",
        "title": "supergrouptitle",
        "username": "supergroupusername",
        "photo": {
            "small_file_id": "smallfileid",
            "big_file_id": "bigfileid"
        },
        "description": "supergroupdescription",
        "invite_link": "supergroupinvitelink",
        "sticker_set_name": "supergroupstickersetname",
        "can_set_sticker_set": true,
        "pinned_message": {
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "supergroup",
                "title": "supergrouptitle",
                "username": "supergroupusername"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "user"
            },
            "text": "test"
        }
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Supergroup(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("supergrouptitle"));
        assert_eq!(chat.username, Some(String::from("supergroupusername")));
        let photo = chat.photo.unwrap();
        assert_eq!(photo.small_file_id, "smallfileid");
        assert_eq!(photo.big_file_id, "bigfileid");
        assert_eq!(chat.description, Some(String::from("supergroupdescription")));
        assert_eq!(chat.invite_link, Some(String::from("supergroupinvitelink")));
        assert_eq!(chat.sticker_set_name, Some(String::from("supergroupstickersetname")));
        assert_eq!(chat.can_set_sticker_set, Some(true));
        assert!(chat.pinned_message.is_some());
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
    let input = json!({
        "id": 1,
        "type": "supergroup",
        "title": "supergrouptitle",
        "username": "supergroupusername"
    });
    let chat: Chat = serde_json::from_value(input).unwrap();
    if let Chat::Supergroup(chat) = chat {
        assert_eq!(chat.id, 1);
        assert_eq!(chat.title, String::from("supergrouptitle"));
        assert_eq!(chat.username, Some(String::from("supergroupusername")));
        assert_eq!(chat.photo.is_none(), true);
        assert_eq!(chat.description.is_none(), true);
        assert_eq!(chat.invite_link.is_none(), true);
        assert_eq!(chat.sticker_set_name.is_none(), true);
        assert_eq!(chat.can_set_sticker_set.is_none(), true);
        assert_eq!(chat.pinned_message.is_none(), true);
    } else {
        panic!("Unexpected chat: {:?}", chat)
    }
}
