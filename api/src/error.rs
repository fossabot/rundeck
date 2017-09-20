use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum ClientError {
    Connectivity,
    InternalClientCreation,
    MalformedUrl,
    UncompatibleVersion,
    TimedOut,
    BadRequest(String)
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ApiError<'a> {
    pub error: bool,
    pub apiversion: i16,
    #[serde(rename="camelCase")]
    pub error_code: Cow<'a, str>,
    pub message: Cow<'a, str>
}

impl<'a> ApiError<'a> {
    pub fn new<U>(error: bool, apiversion: i16, error_code: U, message: U) -> Self
        where U: Into<Cow<'a, str>> {
            Self {
                error,
                apiversion,
                error_code: error_code.into(),
                message: message.into()
            }
        }

    pub fn empty() -> Self {
        Self::new(true, 0, "", "")
    }
}
