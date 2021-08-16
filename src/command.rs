use crate::client;
use crate::page::*;
#[derive(PartialEq)]
pub enum Command {
    Exit,
    My,
    User,
    Matchs,
    Set,
    Main,
    Help,
    Profile,
    Err,
    NotImpletated,
    Some,
}

impl Command {
    pub fn as_str(&self) -> &str {
        match self {
            Command::Exit => "Exit",
            Command::My => "My",
            Command::User => "User",
            Command::Matchs => "Match",
            Command::Set => "Settings",
            Command::Main => "Main",
            Command::Help => "Help",
            Command::Profile => "Profile",
            Command::Err | _ => "Err",
        }
    }
    pub fn help(&self, args: Vec<String>) {
        match self {
            Command::Main => help(args),
            Command::My => my::help(args),
            Command::User => game_record::help(args),
            Command::Matchs => matches::help(args),
            Command::Set => set::help(args),
            Command::Help => println!("help <command> for detailes"),
            Command::Profile => profile::help(args),
            _ => println!("Wrong Command"),
        };
    }
    pub fn run_cli(&self, args: Vec<String>) -> Command {
        let ret: Command = match self {
            Command::Main => {
                println!("back to main");
                Command::Main
            }
            Command::My => my::cli(args),
            Command::User => game_record::cli(args),
            Command::Matchs => matches::cli(args),
            Command::Set => set::cli(args),
            Command::Help => {
                help(args);
                Command::Main
            }
            Command::Profile => profile::cli(args),
            Command::NotImpletated => {
                println!("not impletated");
                Command::NotImpletated
            }
            Command::Err => {
                println!("Wrong Command");
                Command::Err
            }
            Command::Exit => Command::Exit,
            Command::Some => match client::get_history() {
                None => panic!(),
                Some(his) => his.borrow_mut().run_cli(),
            },
        };

        ret
    }
    pub fn from_str(text: &str) -> Command {
        match text.trim() {
            "exit" => Command::Exit,
            "command" | "commands" | "help" => Command::Help,
            "profile" => Command::Profile,
            "User" | "record" => Command::User,
            "match" => Command::Matchs,
            "set" | "setting" => Command::Set,
            "my" => Command::My,
            "main" => Command::Main,
            _ => Command::Err,
        }
    }
}
fn help(mut args: Vec<String>) {
    let command_list = [
        "exit", "commands", "help", "command", "my", "profile", "record", "match", "setting",
    ];
    match args.pop() {
        None => {}
        Some(ar) => {
            let cmd = Command::from_str(&ar);
            match &cmd {
                Command::Main
                | Command::Profile
                | Command::Set
                | Command::Matchs
                | Command::User
                | Command::My => {
                    cmd.help(args);
                }
                _ => {
                    for item in command_list.iter() {
                        println!("{}", item);
                    }
                }
            }
        }
    };
}
