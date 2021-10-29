extern crate lisa;
use lisa::{cli, gui};
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    //TODO : after make gui is_gui defalutly true
    args.pop();
    match args.pop() {
        Some(com) => {
            cli::start_cli(com);
        }
        None => {
            gui();
        }
    };
}
