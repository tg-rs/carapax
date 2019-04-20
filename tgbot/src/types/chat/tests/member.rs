use crate::types::chat::member::ChatMember;

use serde_json::json;

#[test]
fn test_deserialize_chat_member_admin() {
    let admin = json!({
        "status": "administrator",
        "user": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname",
            "last_name": "lastname",
            "username": "username",
            "language_code": "RU"
        },
        "can_be_edited": true,
        "can_change_info": false,
        "can_post_messages": true,
        "can_edit_messages": false,
        "can_delete_messages": true,
        "can_invite_users": false,
        "can_restrict_members": true,
        "can_pin_messages": false,
        "can_promote_members": true
    });
    let admin: ChatMember = serde_json::from_value(admin).unwrap();
    if let ChatMember::Administrator(ref admin) = admin {
        assert_eq!(admin.user.id, 1);
        assert_eq!(admin.user.is_bot, false);
        assert_eq!(admin.user.first_name, String::from("firstname"));
        assert_eq!(admin.user.last_name, Some(String::from("lastname")));
        assert_eq!(admin.user.username, Some(String::from("username")));
        assert_eq!(admin.user.language_code, Some(String::from("RU")));
        assert_eq!(admin.can_be_edited, true);
        assert_eq!(admin.can_change_info, false);
        assert_eq!(admin.can_post_messages, true);
        assert_eq!(admin.can_edit_messages, false);
        assert_eq!(admin.can_delete_messages, true);
        assert_eq!(admin.can_invite_users, false);
        assert_eq!(admin.can_restrict_members, true);
        assert_eq!(admin.can_pin_messages, false);
        assert_eq!(admin.can_promote_members, true);
    } else {
        panic!("Unexpected chat member: {:?}", admin);
    }
}

#[test]
fn test_deserialize_chat_member_creator() {
    let creator = json!({
        "status": "creator",
        "user": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        }
    });
    let creator: ChatMember = serde_json::from_value(creator).unwrap();
    if let ChatMember::Creator(ref creator) = creator {
        assert_eq!(creator.id, 1);
        assert_eq!(creator.is_bot, false);
        assert_eq!(creator.first_name, String::from("firstname"));
        assert_eq!(creator.last_name, None);
        assert_eq!(creator.username, None);
        assert_eq!(creator.language_code, None);
    } else {
        panic!("Unexpected chat member: {:?}", creator);
    }
}

#[test]
fn test_deserialize_chat_member_kicked() {
    let kicked = json!({
        "status": "kicked",
        "user": {
            "id": 1,
            "is_bot": true,
            "first_name": "firstname",
            "last_name": "lastname",
            "username": "username"
        },
        "until_date": 0
    });
    let kicked: ChatMember = serde_json::from_value(kicked).unwrap();
    if let ChatMember::Kicked(ref kicked) = kicked {
        assert_eq!(kicked.user.id, 1);
        assert_eq!(kicked.user.is_bot, true);
        assert_eq!(kicked.user.first_name, String::from("firstname"));
        assert_eq!(kicked.user.last_name, Some(String::from("lastname")));
        assert_eq!(kicked.user.username, Some(String::from("username")));
        assert_eq!(kicked.user.language_code, None);
        assert_eq!(kicked.until_date, 0);
    } else {
        panic!("Unexpected chat member: {:?}", kicked);
    }
}

#[test]
fn test_deserialize_chat_member_left() {
    let left = json!({
        "status": "left",
        "user": {
            "id": 1,
            "is_bot": true,
            "first_name": "firstname"
        }
    });
    let left: ChatMember = serde_json::from_value(left).unwrap();
    if let ChatMember::Left(ref left) = left {
        assert_eq!(left.id, 1);
        assert_eq!(left.is_bot, true);
        assert_eq!(left.first_name, String::from("firstname"));
        assert_eq!(left.last_name, None);
        assert_eq!(left.username, None);
        assert_eq!(left.language_code, None);
    } else {
        panic!("Unexpected chat member: {:?}", left);
    }
}

#[test]
fn test_deserialize_chat_member_plain() {
    let plain = json!({
        "status": "member",
        "user": {
            "id": 1,
            "is_bot": false,
            "first_name": "firstname"
        }
    });
    let plain: ChatMember = serde_json::from_value(plain).unwrap();
    if let ChatMember::Member(ref plain) = plain {
        assert_eq!(plain.id, 1);
        assert_eq!(plain.is_bot, false);
        assert_eq!(plain.first_name, String::from("firstname"));
        assert_eq!(plain.last_name, None);
        assert_eq!(plain.username, None);
        assert_eq!(plain.language_code, None);
    } else {
        panic!("Unexpected chat member: {:?}", plain);
    }
}

#[test]
fn test_deserialize_chat_member_restricted() {
    let restricted = json!({
        "status": "restricted",
        "user": {
            "id": 1,
            "is_bot": true,
            "first_name": "firstname"
        },
        "until_date": 0,
        "can_send_messages": true,
        "can_send_media_messages": false,
        "can_send_other_messages": true,
        "can_add_web_page_previews": false,
        "is_member": true
    });
    let restricted: ChatMember = serde_json::from_value(restricted).unwrap();
    if let ChatMember::Restricted(ref restricted) = restricted {
        assert_eq!(restricted.user.id, 1);
        assert_eq!(restricted.user.is_bot, true);
        assert_eq!(restricted.user.first_name, String::from("firstname"));
        assert_eq!(restricted.user.last_name, None);
        assert_eq!(restricted.user.username, None);
        assert_eq!(restricted.user.language_code, None);
        assert_eq!(restricted.until_date, 0);
        assert_eq!(restricted.can_send_messages, true);
        assert_eq!(restricted.can_send_media_messages, false);
        assert_eq!(restricted.can_send_other_messages, true);
        assert_eq!(restricted.can_add_web_page_previews, false);
        assert_eq!(restricted.is_member, true);
    } else {
        panic!("Unexpected chat member: {:?}", restricted);
    }
}
