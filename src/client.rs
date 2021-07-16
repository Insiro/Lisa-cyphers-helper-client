use crate::object::charactor::CharList;
use crate::object::clan;
use crate::object::player;

use crate::util::{data_serializer::date_se, temp};
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

type UtcTime = DateTime<Utc>;

type RcClient = Rc<RefCell<Client>>;

static mut STATIC_CLIENT: Option<RcClient> = None;
static mut REQ_CLIENT: Option<Rc<RefCell<reqwest::Client>>> = None;

#[derive(Serialize, Deserialize)]
pub struct Client {
    //setting data
    pub auto_start: bool,
    pub is_nexon: bool,
    //news data
    #[serde(with = "date_se")]
    last_notify: Option<UtcTime>,
    #[serde(with = "date_se")]
    last_magazine: Option<UtcTime>,
    #[serde(with = "date_se")]
    last_update: Option<UtcTime>,
    // user data
    players: Vec<player::PlayerBase>,
    #[serde(with = "date_se")]
    birth_day: Option<UtcTime>,
    clans: Vec<clan::ClanBase>,
    charactors: CharList,
    neople_api_key: String,
}

fn empty_client() -> Client {
    Client {
        auto_start: false,
        is_nexon: true,
        players: Vec::new(),
        birth_day: None,
        charactors: Vec::new(),
        clans: Vec::new(),
        last_notify: None,
        last_magazine: None,
        last_update: None,
        neople_api_key: "".to_string(),
    }
}

fn new_client() -> Client {
    let pstr = temp::get_config_path();
    let path = Path::new(pstr.as_str());
    let client: Client;
    let serial: String;
    if path.exists() {
        serial = fs::read_to_string(path).expect("failed to open config file");
        client = serde_json::from_str(&serial).expect("failed to DeSerialize");
    } else {
        let mut conf = fs::File::create(path).expect("failed to create config file");
        client = empty_client();
        conf.write_all(
            serde_json::to_string(&client)
                .expect("faild to serialize")
                .as_bytes(),
        )
        .expect("failed to save file");
    }
    println!("{}", serde_json::to_string(&client).expect("err"));
    client
}

pub fn init() {
    unsafe {
        REQ_CLIENT = Some(Rc::new(RefCell::new(reqwest::Client::new())));
        STATIC_CLIENT = Some(Rc::new(RefCell::new(new_client())));
    }
}
fn get_client() -> Option<RcClient> {
    unsafe {
        match &STATIC_CLIENT {
            Some(rcs) => Some(Rc::clone(&rcs)),
            None => None,
        }
    }
}
fn get_req_client() -> Option<Rc<RefCell<reqwest::Client>>> {
    unsafe {
        match &REQ_CLIENT {
            Some(req) => Some(Rc::clone(&req)),
            None => None,
        }
    }
}
impl Client {
    fn get_api_key(&self) -> &str {
        &self.neople_api_key
    }
    fn open_game_page(&self) {
        //TODO: open webBrowser
    }
    fn get_payers(&mut self) -> &mut Vec<player::PlayerBase> {
        &mut self.players
    }
    fn get_clans(&mut self) -> &mut Vec<clan::ClanBase> {
        &mut self.clans
    }
    fn set_home_type(&mut self, is_nexon: bool) {
        self.is_nexon = is_nexon
    }
    fn get_birth_day(&self) -> &Option<UtcTime> {
        &self.birth_day
    }
    fn get_charactors(&mut self) -> &CharList {
        &self.charactors
    }
}
