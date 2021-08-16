extern crate lisa;
use lisa::{cli, client, command, gui};
use std::env;
#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    //TODO : after make gui is_gui defalutly true
    args.pop();
    client::init();
    match args.pop() {
        Some(com) => match com.as_str() {
            "--load" => {
                cli::load(args);
                return;
            }
            "--cli" => {
                client::start_msg();
                match args.pop() {
                    Some(arg) => {
                        command::Command::from_str(&arg).run_cli(args);
                    }
                    None => cli::cli(),
                }
            }
            _ => {
                println!("wrong command : {}", com);
                return;
            }
        },
        None => {
            gui();
        }
    };
}
