use crate::command::Command;
use std::{io, process};
pub trait CliTrait {
    fn cli_runner(&mut self) -> Command;
}

pub fn start_cli(com: String) {
    match com.as_str() {
        "--load" => {
            //TODO: load data
            return;
        }
        "--cli" => {
            let mut cli = CliController::new();
            cli.start();
        }
        _ => {
            println!("wrong command : {}", com);
            return;
        }
    }
}

struct CliController {
    command_env: Command,
    env: Box<dyn CliTrait>,
}
impl CliController {
    pub fn new() -> CliController {
        let cli = CliController {
            command_env: Command::Main,
            env: Box::new(MainPage::new()),
        };
        return cli;
    }
    pub fn start(&mut self) {
        loop {
            self.cmd_runner();
        }
    }
    fn cmd_runner(&mut self) {
        if &self.command_env == &Command::Exit {
            process::exit(0);
        }
        self.print_env();
        self.env.cli_runner();
    }
    fn print_env(&self) {
        println!("{} > ", self.command_env.as_str());
        //TODO: print Currunt Env Enformation
    }
}
struct MainPage {
    command_env: Command,
}
impl MainPage {
    pub fn new() -> MainPage {
        return MainPage {
            command_env: Command::Main,
        };
    }
    fn set_command(&mut self) {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("error read");
        line = line.to_lowercase();
        line = line.trim().to_string();
        let coms: Vec<&str> = line.split(" ").collect();
        self.command_env = Command::from_str(coms[0]);
    }
}
impl CliTrait for MainPage {
    fn cli_runner(&mut self) -> Command {
        self.set_command();
        return self.command_env.clone();
    }
}
pub struct Cli();
pub fn load(_args: Vec<String>) {}
