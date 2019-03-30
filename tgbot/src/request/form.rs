use crate::types::{InputFile, InputFileInfo, InputFileKind, InputFileReader};
use hyper_multipart_rfc7578::client::multipart::Form as MultipartForm;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) enum FormValue {
    Text(String),
    File(InputFile),
}

impl<T> From<T> for FormValue
where
    T: ToString,
{
    fn from(value: T) -> Self {
        FormValue::Text(value.to_string())
    }
}

impl From<InputFile> for FormValue {
    fn from(value: InputFile) -> Self {
        FormValue::File(value)
    }
}

#[derive(Debug)]
pub(crate) struct Form {
    fields: HashMap<String, FormValue>,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self { fields: HashMap::new() }
    }

    pub(crate) fn set_field<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<FormValue>,
    {
        self.fields.insert(name.into(), value.into());
    }
}

impl From<Form> for MultipartForm<'static> {
    fn from(form: Form) -> Self {
        let mut result = MultipartForm::default();
        for (field_name, field_value) in form.fields {
            match field_value {
                FormValue::Text(text) => result.add_text(field_name, text),
                FormValue::File(file) => match file.kind {
                    InputFileKind::Path(path) => result.add_file(field_name, path).unwrap(),
                    InputFileKind::Reader(InputFileReader {
                        reader,
                        info: file_info,
                    }) => match file_info {
                        Some(InputFileInfo {
                            name: file_name,
                            mime_type: Some(mime_type),
                        }) => result.add_reader_file_with_mime(field_name, reader, file_name, mime_type),
                        Some(InputFileInfo {
                            name: file_name,
                            mime_type: None,
                        }) => result.add_reader_file(field_name, reader, file_name),
                        None => result.add_reader(field_name, reader),
                    },
                    InputFileKind::Id(file_id) => result.add_text(field_name, file_id),
                    InputFileKind::Url(url) => result.add_text(field_name, url),
                },
            }
        }
        result
    }
}
