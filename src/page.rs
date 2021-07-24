pub mod game_record;
pub mod matches;
pub mod my;
pub mod profile;
pub mod set;

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
}

pub trait CLI {
    fn run_cli(args: Vec<String>) -> Command;
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
            Command::Err => "Err",
        }
    }
    pub fn help(&self, args: Vec<String>) {
        match self {
            Command::Exit | Command::Main => {
                println!("back to main")
            }
            Command::My => my::help(args),
            Command::User => game_record::help(args),
            Command::Matchs => matches::help(args),
            Command::Set => set::help(args),
            Command::Help => println!("help <command> for detailes"),
            Command::Profile => profile::help(args),
            Command::Err => println!("Wrong Command"),
        };
    }
    pub fn run_cli(&self, args: Vec<String>) -> Command {
        match self {
            Command::Exit | Command::Main => {
                println!("back to main")
            }
            Command::My => my::cli(args),
            Command::User => game_record::cli(args),
            Command::Matchs => matches::cli(args),
            Command::Set => set::cli(args),
            Command::Help => help(args),
            Command::Profile => profile::cli(args),
            Command::Err => println!("Wrong Command"),
        };
        Command::Err //TODO: get result from run cli and return
    }
    pub fn from_string(text: String) -> Command {
        match text.as_str() {
            "exit" => Command::Exit,
            "commands" | "help" => Command::Help,
            "profile" => Command::Profile,
            "User" | "record" => Command::User,
            "match" => Command::Matchs,
            "set" | "setting" => Command::Set,
            "my" => Command::My,
            _ => Command::Err,
        }
    }
    pub fn from_str(text: &str) -> Command {
        match text {
            "exit" => Command::Exit,
            "commands" | "help" => Command::Help,
            "profile" => Command::Profile,
            "User" | "record" => Command::User,
            "match" => Command::Matchs,
            "set" | "setting" => Command::Set,
            "my" => Command::My,
            _ => Command::Err,
        }
    }
}

fn help(mut args: Vec<String>) {
    let command_list = [
        "exit",
        "commands | help",
        "my",
        "profile",
        "record",
        "match",
        "setting",
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
