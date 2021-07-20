use std::io;

use crate::page::{game_record, matches, my, profile, set, Command};
use crate::util::history;

pub fn cli_arg(arg1: String, args: Vec<String>) -> bool {
    Command::from_str(&arg1).run_cli(args);
    false
}

pub fn cli() {
    let mut cmd: Command = Command::Main;
    while cmd != Command::Exit {
        let mut args = get_args();
        let fst = match args.pop() {
            None => {
                continue;
            }
            Some(it) => it,
        };
        cmd = Command::from_str(&fst);
        cmd = cmd.run_cli(args);
    }
}

fn get_args() -> Vec<String> {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("error read");
    line = line.to_lowercase();
    let mut args: Vec<String> = Vec::new();
    for item in line.split(" ") {
        args.push(item.to_string());
    }
    args.reverse();
    args
}

pub fn start(mut args: Vec<String>) {
    match args.pop() {
        Some(arg) => {
            Command::from_str(&arg).run_cli(args);
        }
        None => cli(),
    }
}
