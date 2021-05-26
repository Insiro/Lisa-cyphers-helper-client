pub type ParseResult<T> = Result<T, ParsingError>;
pub type ParseResults = Result<String, ParsingError>;
pub struct ParsingError {
    msg: String,
    error: ParsingErrorKind,
}
pub enum ParsingErrorKind {
    UserNotFoundError,
    NetworkError,
    NotDefinedError,
    DataError,
}
pub fn new(msg: String, error: ParsingErrorKind) -> ParsingError {
    ParsingError { msg, error }
}
impl ParsingError {
    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn get_error(&self) -> &ParsingErrorKind {
        &self.error
    }
}
