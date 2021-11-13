pub mod charactor;
pub mod clan;
pub mod neople;
pub mod saved;
pub mod notify;
pub mod player;
pub mod info;
pub mod record;


pub trait SavedObject : Object {
    fn save();
    fn load();
    
}
pub trait Object{
}