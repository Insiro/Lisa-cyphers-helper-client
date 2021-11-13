pub mod client;
pub mod servicer;
pub mod object;
pub mod cli;
pub mod command;
pub mod error;
pub mod page;
pub mod util;
extern crate getset;

pub fn gui() {
    client::Client::new();
}
