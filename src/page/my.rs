use crate::client::get_client;
use crate::command::Command;

pub fn cli(mut _arg: Vec<String>) -> Command {
    let mut _client = get_client();
    Command::NotImpletated
}
pub fn help(_args: Vec<String>) {}
