use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

//#region redefine module and NameSpace
mod base;
mod info;
mod record;
mod saved;

pub use base::Base;
pub use info::Info;
pub use record::Records;
pub use saved::Saved;

pub mod builder {
    pub use super::base::BaseBuilder;
    pub use super::info::InfoBuilder;
    pub use super::record::RecordBuilder;
    pub use super::saved::SavedBuilder;
}
//#endregion
pub trait Player: super::Object {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_grade(&self) -> u8;
}

trait PlayerBuilder: Sized {
    type Player;
    fn new(id: String) -> Result<Self, Box<dyn std::error::Error>>;
    fn build(&self) -> Result<Self::Player, Box<dyn std::error::Error>>;
}

#[macro_export]
macro_rules! player_impl_neople {
    ($type:ident) => {
        impl Player for $type {
            fn get_id(&self) -> &str {
                &self.playerId
            }
            fn get_name(&self) -> &str {
                &self.nickname
            }
            fn get_grade(&self) -> u8 {
                self.grade
            }
        }
    };
}
