use crate::{
    methods::Method,
    request::{Form, Request},
    types::{File, InputFile, Integer},
};

/// Upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods
#[derive(Debug)]
pub struct UploadStickerFile {
    form: Form,
}

impl UploadStickerFile {
    /// Creates a new UploadStickerFile
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker file owner
    /// * png_sticker - Png image with the sticker, must be up to 512 kilobytes in size,
    ///                 dimensions must not exceed 512px, and either width or height must be exactly 512px
    pub fn new<P>(user_id: Integer, png_sticker: P) -> Self
    where
        P: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("user_id", user_id);
        form.insert_field("png_sticker", png_sticker.into());
        UploadStickerFile { form }
    }
}

impl Method for UploadStickerFile {
    type Response = File;

    fn into_request(self) -> Request {
        Request::form("uploadStickerFile", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn add_sticker_to_set() {
        let request = UploadStickerFile::new(1, InputFile::file_id("sticker-id")).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/uploadStickerFile"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["user_id"].get_text().unwrap(), "1");
            assert!(form.fields["png_sticker"].get_file().is_some());
        } else {
            panic!("Unexpected request body");
        }
    }
}
