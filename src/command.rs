use crate::client;
use crate::page::*;
#[derive(PartialEq, Clone, Copy)]
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
    Back,
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
            Command::Back => "Back",
            Command::Some => "Some",
            Command::NotImpletated => "NotImplied",
            Command::Err => "Err",
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
            Command::Back => println!("return to previous level"),
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
            Command::Exit => Command::Exit,
            Command::Some => match client::get_history() {
                None => panic!(),
                Some(his) => his.borrow_mut().run_cli(),
            },
            Command::Back => Command::Back,
            Command::Err => {
                println!("Wrong Command");
                Command::Err
            }
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
            "back" => Command::Back,
            _ => Command::Err,
        }
    }
}
fn help(mut args: Vec<String>) {
    match args.pop() {
        None => help_sub(),
        Some(ar) => {
            let cmd = Command::from_str(&ar);
            match &cmd {
                Command::Main
                | Command::Profile
                | Command::Set
                | Command::Matchs
                | Command::User
                | Command::My => cmd.help(args),
                _ => {
                    if cmd == Command::Err && "Info" == ar.trim() {
                        maker_info();
                        return;
                    };
                    help_sub();
                }
            };
        }
    };
}
fn help_sub() {
    let items = [
        "Exit",
        "My",
        "User",
        "Matchs",
        "Set",
        "Main",
        "Help",
        "Profile",
        "Back",
        "help Info",
    ];
    println!("Lisa : Cyphers Helper");
    println!("auto game history search | manage friends profile | clan information");
    println!("-Command List-");
    for item in items.iter() {
        println!("{}", item);
    }
}
fn maker_info() {
    println!("Project Lisa");
    println!("Lisa: Cyphers Helper (client ver)");
    println!("\thttps://github.com/Insiro/Lisa-cyphers-helper-client");
    println!("Developed by Insiro :\thttps://github.com/Insiro");
    println!("Project Lisa : \thttps://lisa.Insiro.me/");
    println!("------------------------------------------------------------");
    println!("Powered by neople api : \nhttps://developers.neople.co.kr/");
}
