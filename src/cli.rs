use crate::command::Command;
use std::io;
pub trait CLI {
    fn cli(args: Vec<String>) -> Command;
    fn help(args: Vec<String>);
    fn cli_main();
    fn comm() -> Command;
}

pub fn cli_arg(arg1: String, args: Vec<String>) -> bool {
    Command::from_str(&arg1).run_cli(args);
    false
}

pub fn cli() {
    let mut cmd: Command = Command::Main;
    while cmd != Command::Exit {
        print!("{} > ", cmd.as_str());
        let mut args = get_args();
        let fst = match args.pop() {
            None => {
                println!("wrong input");
                continue;
            }
            Some(it) => it,
        };
        if cmd == Command::Err {
            cmd = Command::from_str(&fst);
        }
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

pub struct Cli();
pub fn load(_args: Vec<String>) {}
