#[allow(non_snake_case)]
use std::fmt::Display;

pub trait Builder: Sized {
    type Target;
    fn new() -> Self;
    fn build(&mut self) -> std::result::Result<Self::Target, Box<dyn std::error::Error>>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}
impl std::error::Error for Error {}
