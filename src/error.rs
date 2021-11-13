#[allow(non_snake_case)]

pub type Result<T> = std::result::Result<T, Error>;
use std::fmt::Display;
#[derive(Debug)]
pub enum Kind {
    NotFoundError,
    NetworkError,
    NotDefined,
    DataError,
    FileLoadError,
    DataLoadError,
}
impl Kind {
    pub fn to_str(&self) -> &str {
        match self {
            Kind::NotFoundError => "NotFoundError",
            Kind::NetworkError => "NetworkError",
            Kind::NotDefined => "NotDefined",
            Kind::DataError => "DataError",
            Kind::FileLoadError => "FileLoadError",
            Kind::DataLoadError => "FileLoadError",
        }
    }
}
pub fn new(msg: &str, error: Kind) -> Error {
    Error {
        msg: msg.to_string(),
        error,
    }
}
#[derive(Debug)]
pub struct Error {
    msg: String,
    error: Kind,
}

impl Error {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn get_error(&self) -> &Kind {
        &self.error
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error.to_str())
    }
}
impl std::error::Error for Error {}
