use crate::page::{game_record, matches, profile, set};
use std::io;

enum commands {
    Exit,
    My,
    User,
    Matchs,
    Setting,
}

impl commands {
    fn as__str(&self) -> &str {
        match self {
            commands::Exit => "Exit",
            commands::My => "My",
            commands::User => "User",
            commands::Matchs => "Match",
            commands::Setting => "Settings",
        }
    }
}

pub fn start() {
    let mut line = String::new();
    let mut command = 0;
    while command != -1 {
        io::stdin().read_line(&mut line);
        let cm = line.to_lowercase();
        let mut args: Vec<&str>;
        args = cm.split(" ").collect();
        match args.get(0) {
            None => {
                println!("wrong command\ttry commands");
            }
            Some(ar) => match *ar {
                "exit" => return,
                "commands" | "help" => help(),
                "profile" => profile::cli(&mut args, 2),
                "record" => game_record::cli(&mut args, 2),
                "match" => matches::cli(&mut args, 2),
                "set" | "setting" => set::cli(&mut args, 2),
                // "my" => my::cli(&mut args,0),
                _ => {
                    println!("wrong command\ttry commands");
                }
            },
        }
        command = -1;
    }
}

fn help() {
    let command_list = [
        "exit",
        "commands | help",
        "my",
        "profile",
        "record",
        "match",
        "setting",
    ];
    for item in command_list.iter() {
        println!("{}", item);
    }
}
