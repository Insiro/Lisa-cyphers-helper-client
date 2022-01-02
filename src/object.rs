pub mod builder;
pub mod charactor;
pub mod clan;
pub mod info;
pub mod neople;
pub mod notify;
pub mod player;
pub mod record;
pub mod saved;

pub trait SavedObject: Object {
    fn save();
    fn load();
}

pub trait Object {}
