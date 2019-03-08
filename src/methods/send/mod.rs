mod animation;
mod audio;
mod chat_action;
mod contact;
mod document;
mod invoice;
mod location;
mod media_group;
mod photo;
mod venue;
mod video;
mod video_note;
mod voice;

pub use self::{
    animation::*, audio::*, chat_action::*, contact::*, document::*, invoice::*, location::*, media_group::*, photo::*,
    venue::*, video::*, video_note::*, voice::*,
};
