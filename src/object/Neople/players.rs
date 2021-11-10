#![allow(non_snake_case)]
use super::info;
pub mod Base;
pub mod Info;
pub mod Records;

trait Player {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
}

trait PlayerBuilder: Sized {
    fn new(id: String) -> Self;
    fn build(&self) -> Result<Box<dyn Player>, Box<dyn std::error::Error>>;
}

#[macro_export]
macro_rules! player_impl {
    ($type:ident) => {
        impl Player for $type {
            fn get_id(&self) -> &str {
                &self.playerId
            }
            fn get_name(&self) -> &str {
                &self.nickname
            }
        }
    };
}
