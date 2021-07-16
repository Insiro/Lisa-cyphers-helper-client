extern crate lisa;
use lisa::{cli, client, gui};
use std::env;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1);
    //TODO : after make gui is_gui defalutly true
    let mut is_gui = false;
    match query {
        Some(com) => {
            match com.as_str() {
                "load" => {
                    load(&args);
                    return;
                }
                "cli" => {
                    is_gui = false;
                }
                _ => return,
            };
        }
        None => {}
    };
    client::init();
    if is_gui {
        gui();
    } else {
        cli::start();
    }
}
fn load(_args: &Vec<String>) {
    //TODO : load datas and save -> move to lib
}
