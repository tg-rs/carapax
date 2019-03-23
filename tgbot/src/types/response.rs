use crate::types::primitive::Integer;
use serde::{de::Error, Deserialize, Deserializer};

/// API Response
#[derive(Clone, Debug)]
pub enum Response<T> {
    /// Success
    Success(T),
    /// Error
    Error(ResponseError),
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        };
        Ok(if raw.ok {
            Response::Success(required!(result))
        } else {
            Response::Error(ResponseError {
                description: required!(description),
                error_code: raw.error_code,
                parameters: raw.parameters,
            })
        })
    }
}

/// Response error
#[derive(Clone, Debug, failure::Fail)]
#[fail(
    display = "A telegram error has occurred: code={:?} message={}",
    error_code, description
)]
pub struct ResponseError {
    /// Human-readable description
    pub description: String,
    /// Error code
    pub error_code: Option<Integer>,
    /// Parameters
    pub parameters: Option<ResponseParameters>,
}

/// Contains information about why a request was unsuccessful
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier
    pub migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control,
    /// the number of seconds left to wait
    /// before the request can be repeated
    pub retry_after: Option<Integer>,
}

#[derive(Clone, Debug, Deserialize)]
struct RawResponse<T> {
    ok: bool,
    description: Option<String>,
    error_code: Option<Integer>,
    result: Option<T>,
    parameters: Option<ResponseParameters>,
}
