use std::borrow::Cow;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ClientError {
    Connectivity,
    InternalClientCreation,
    MalformedUrl,
    UncompatibleVersion,
    TimedOut,
    BadRequest(String),
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

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::InternalClientCreation => write!(f, "Can't create an internal http client"),
            ClientError::MalformedUrl => write!(f, "RUNDECK_URL is malformed"),
            ClientError::UncompatibleVersion => write!(f, "RUNDECK_URL doesn't contain a valid API_VERSION"),
            _ => write!(f, "SuperError is here!")
        }
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        ""
    }
}
