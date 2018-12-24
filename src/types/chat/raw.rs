use crate::types::chat::photo::ChatPhoto;
use crate::types::message::Message;
use crate::types::primitive::Integer;
use crate::types::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawChat {
    pub id: Integer,
    #[serde(rename = "type")]
    pub kind: RawChatKind,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum RawChatKind {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    Supergroup,
    #[serde(rename = "channel")]
    Channel,
}

#[derive(Debug, Deserialize)]
pub struct RawChatMember {
    pub user: User,
    pub status: RawChatMemberStatus,
    pub until_date: Option<Integer>,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum RawChatMemberStatus {
    #[serde(rename = "administrator")]
    Administrator,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "kicked")]
    Kicked,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "restricted")]
    Restricted,
}
