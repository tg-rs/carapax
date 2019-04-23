mod animation;
mod audio;
mod callback_query;
mod chat;
mod contact;
mod document;
mod file;
mod game;
mod inline_mode;
mod input_media;
mod location;
mod message;
mod passport;
mod payments;
mod photo_size;
mod poll;
mod primitive;
mod reply_markup;
mod response;
mod stickers;
mod update;
mod user;
mod venue;
mod video;
mod video_note;
mod voice;

pub use self::{
    animation::*, audio::*, callback_query::*, chat::*, contact::*, document::*, file::*, game::*, inline_mode::*,
    input_media::*, location::*, message::*, passport::*, payments::*, photo_size::*, poll::*, primitive::*,
    reply_markup::*, response::*, stickers::*, update::*, user::*, venue::*, video::*, video_note::*, voice::*,
};
