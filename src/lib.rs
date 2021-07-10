pub fn cli() {}
pub fn gui() {}
pub mod client;
pub mod servicer;
pub mod object {
    pub mod charactor;
    pub mod clan;
    pub mod notify;
    pub mod player;
}
pub mod error;
pub mod exs;
pub mod page;
//TODO: make exact
pub mod data_serializer;
pub mod temp;
