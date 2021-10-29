pub mod client;
pub mod servicer;
pub mod object {
    pub mod charactor;
    pub mod clan;
    pub mod neople;
    pub mod notify;
    pub mod player;
    pub trait Objects {
        fn refrash(&mut self) -> super::error::Result<()>;
    }
}
pub mod cli;
pub mod command;
pub mod error;
pub mod page;
pub mod util;
extern crate getset;

pub fn gui() {
    client::init();
}