use crate::{methods::Method, request::Request, types::File};
use serde::Serialize;

/// Get basic info about a file and prepare it for downloading
///
/// For the moment, bots can download files of up to 20MB in size
///
/// The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>,
/// where <file_path> is taken from the response
///
/// It is guaranteed that the link will be valid for at least 1 hour
///
/// When the link expires, a new one can be requested by calling getFile again
///
/// Note: This function may not preserve the original file name and MIME type
/// You should save the file's MIME type and name (if available) when the File object is received
#[derive(Clone, Debug, Serialize)]
pub struct GetFile {
    file_id: String,
}

impl GetFile {
    /// Creates a new GetFile
    ///
    /// # Arguments
    ///
    /// * file_id - File identifier to get info about
    pub fn new<S: Into<String>>(file_id: S) -> Self {
        GetFile {
            file_id: file_id.into(),
        }
    }
}

impl Method for GetFile {
    type Response = File;

    fn into_request(self) -> Request {
        Request::json("getFile", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_file() {
        let request = GetFile::new("file-id").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getFile");
        if let RequestBody::Json(data) = request.into_body() {
            assert_eq!(data.unwrap(), r#"{"file_id":"file-id"}"#);
        } else {
            panic!("Unexpected request body");
        }
    }
}
