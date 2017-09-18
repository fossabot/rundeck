#[derive(Debug, PartialEq)]
pub enum ClientError {
    Connectivity,
    InternalClientCreation,
    MalformedUrl,
    UncompatibleVersion,
    TimedOut,
    BadRequest(String)
}
