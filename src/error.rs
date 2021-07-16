pub mod ParseError {
    pub type PResult<T> = Result<T, Error>;
    pub struct Error {
        msg: String,
        error: Kind,
    }
    pub enum Kind {
        UserNotFoundError,
        NetworkError,
        NotDefinedError,
        DataError,
    }
    pub fn new(msg: String, error: Kind) -> Error {
        Error { msg, error }
    }
    impl Error {
        pub fn get_msg(&self) -> &str {
            &self.msg
        }
        pub fn get_error(&self) -> &Kind {
            &self.error
        }
    }
}
pub mod LoadError {
    pub struct Error {
        msg: String,
        error: Kind,
    }
    pub enum Kind {
        FileError,
        DataError,
    }
    impl Error {
        pub fn get_msg(&self) -> &str {
            &self.msg
        }
        pub fn get_error(&self) -> &Kind {
            &self.error
        }
    }
}
