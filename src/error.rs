#[allow(non_snake_case)]

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error {
    msg: String,
    error: Kind,
}
pub enum Kind {
    UserNotFoundError,
    NetworkError,
    NotDefinedError,
    DataError,
    FileLoadError,
    DataLoadError,
}
pub fn new(msg: &str, error: Kind) -> Error {
    Error {
        msg: msg.to_string(),
        error,
    }
}
impl Error {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn get_error(&self) -> &Kind {
        &self.error
    }
}
