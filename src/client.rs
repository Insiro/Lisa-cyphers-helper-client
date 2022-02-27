use crate::{
    gui::GUIUi,
    util::{history, temp},
};
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::sync::RwLock;
use std::{cell::RefCell, fs};

mod meta;
mod setting;
mod subscription;
#[cfg(test)]
mod test;
mod userinfo;
type RwLockCLient = Rc<RefCell<RwLock<Client>>>;
static mut HISTORY: Option<Rc<RefCell<history::Histories>>> = None;
static mut CLIENT: Option<RwLockCLient> = None;
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
        file.sync_data()?;
        return Ok(());
    }
    fn default() -> Self;
    fn get_default_path() -> String;
    fn print_start_msg(&self);
    fn set_path(&mut self, new_path: &str) -> Result<(), ()>;
}

pub struct Client {
    pub setting: setting::Setting,
    meta: meta::Meta,
    pub user: userinfo::UserInfo,
    pub subscription: subscription::Subscription,
    pub config_path: String,
    app: GUIUi,
}
impl Client {
    pub fn get_ui_controller(&mut self) -> &GUIUi {
        &self.app
    }
    pub fn new() -> Self {
        let setting_path_str = temp::get_config_path();
        let setting_path = Path::new(setting_path_str.as_str());
        let setting = setting::Setting::new(setting_path);
        let raw_path = setting.get_conf_path().to_string();
        let config_path = Path::new(&raw_path);

        Self {
            setting,
            meta: meta::Meta::new(),
            user: userinfo::UserInfo::new(config_path),
            subscription: subscription::Subscription::new(config_path),
            config_path: raw_path,
            app: crate::gui::init(),
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
    #[allow(dead_code)]
    fn open_game_page(&self) {
        todo!();
        //TODO: open webBrowser
    }
    pub fn instance() -> RwLockCLient {
        unsafe {
            match &CLIENT {
                None => {
                    let client = Rc::new(RefCell::new(RwLock::new(Client::new())));
                    let ref_client = Rc::clone(&client);
                    CLIENT = Some(client);
                    return ref_client;
                }
                Some(client) => Rc::clone(client),
            }
        }
    }

    pub fn start_msg(&self) {
        println!("start as cli");
        println!("---setting---");
        println!("lisa - version : {}", self.meta.get_version());

        println!("access as nexon : {}", self.setting.is_nexon);
        println!("----feed infos----");
        //TODO: print last update magazine, update, notify as date
        // println!("magazine : {}", client.get_last_magazine());
        println!("---------");
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
