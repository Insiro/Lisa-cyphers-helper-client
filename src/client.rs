use crate::util::{history, temp};
use cargo_metadata::MetadataCommand;
use reqwest;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::{cell::RefCell, fs};

mod setting;
mod subscription;
#[cfg(test)]
mod test;
mod userinfo;

type RcClient = Rc<RefCell<Client>>;

static mut STATIC_CLIENT: Option<RcClient> = None;
static mut REQ_CLIENT: Option<Rc<RefCell<reqwest::Client>>> = None;
static mut HISTORY: Option<Rc<RefCell<history::Histories>>> = None;

trait Save: Sized {
    fn new(path: &Path) -> Self;
    fn save(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>>;
}
trait ClientSave: Serialize {
    fn create(data: &Self, path: &Path) {
        match Self::write_file(data, path) {
            Err(_) => {
                let d_path = Self::get_default_path();
                let path = Path::new(&d_path);
                Self::write_file(data, path).expect("failed to save data");
            }
            _ => {}
        };
    }
    fn write_file(data: &Self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::create(path)?;
        file.write_all(serde_json::to_string(&data)?.as_bytes())?;
        return Ok(());
    }
    fn default() -> Self;
    fn get_default_path() -> String;
    fn print_start_msg(&self);
}
pub struct Client {
    pub setting: setting::Setting,
    meta: ClientMeta,
    pub user: userinfo::UserInfo,
    pub subscription: subscription::Subscription,
    pub config_path: String,
}
impl Client {
    pub fn new() -> Self {
        let setting_path_str = temp::get_config_path();
        let setting_path = Path::new(setting_path_str.as_str());
        let setting = setting::Setting::new(setting_path);
        let raw_path = (&setting.conf_path).to_string();
        let config_path = Path::new(&raw_path);

        Self {
            setting,
            meta: ClientMeta::new(),
            user: userinfo::UserInfo::new(config_path),
            subscription: subscription::Subscription::new(config_path),
            config_path: raw_path,
        }
    }
    pub fn print_start_msg(&self) {
        println!("start as cli");
        println!("---setting---");
        println!("lisa - version : {}", self.meta.get_version());
        self.setting.print_start_msg();
        println!("----feed infos----");
        self.subscription.print_start_msg();
        println!("---------");
    }
}

pub struct ClientMeta {
    version: String,
}
impl Default for ClientMeta {
    fn default() -> Self {
        return Self::new();
    }
}

impl ClientMeta {
    pub fn new() -> Self {
        //TODO: check It work
        return Self {
            version: Self::load(),
        };
    }
    fn get_version(&self) -> &str {
        &self.version
    }
    fn load() -> String {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .current_dir(&path)
            .exec()
            .unwrap();
        let root = meta.root_package().unwrap();
        let version = (&root.version).to_string();
        return version;
    }
}

pub fn start_msg() {
    let client = match get_client() {
        None => panic!(),
        Some(a) => a,
    };
    println!("start as cli");
    println!("---setting---");
    println!("lisa - version : {}", client.borrow().meta.get_version());

    println!("access as nexon : {}", client.borrow().setting.is_nexon);
    println!("----feed infos----");
    //TODO: print last update magazine, update, notify as date
    // println!("magazine : {}", client.get_last_magazine());
    println!("---------");
}
pub fn init() {
    unsafe {
        REQ_CLIENT = Some(Rc::new(RefCell::new(reqwest::Client::new())));
        STATIC_CLIENT = Some(Rc::new(RefCell::new(Client::new())));
        HISTORY = Some(Rc::new(RefCell::new(history::init())));
    }
}
pub fn get_client() -> Option<RcClient> {
    unsafe {
        match &STATIC_CLIENT {
            Some(rcs) => Some(Rc::clone(&rcs)),
            None => None,
        }
    }
}
pub fn get_req_client() -> Option<Rc<RefCell<reqwest::Client>>> {
    unsafe {
        match &REQ_CLIENT {
            Some(req) => Some(Rc::clone(&req)),
            None => None,
        }
    }
}
pub fn get_history() -> Option<Rc<RefCell<history::Histories>>> {
    unsafe {
        match &HISTORY {
            None => None,
            Some(his) => Some(Rc::clone(&his)),
        }
    }
}

//TODO: remove dead_code notation
/*
#[allow(dead_code)]
impl Client {
    fn open_game_page(&self) {
        //TODO: open webBrowser
    }
    fn get_payers(&mut self) -> &mut Vec<player::PlayerBase> {
        &mut self.players
    }
    fn get_clans(&mut self) -> &mut Vec<clan::ClanBase> {
        &mut self.clans
    }
    fn get_birth_day(&self) -> &Option<UtcTime> {
        &self.birth_day
    }
}
*/
