use webbrowser;

use crate::error::{self, ParseResult, ParsingErrorKind};

use reqwest;
use select::document::Document;
use select::predicate::Class;

pub struct ClanBase {
    name: String,
    post_url: Option<String>,
}
pub trait Clan {
    fn get_clan(&self) -> &Option<ClanBase>;
    fn set_clan(&mut self, clan: Option<ClanBase>);
    fn remove_clan(&mut self);
}

impl ClanBase {
    pub fn dumy() -> ClanBase {
        ClanBase {
            name: "".to_string(),
            post_url: None,
        }
    }
    pub fn new(name: String, post_url: Option<String>) -> ClanBase {
        ClanBase { name, post_url }
    }
    pub async fn get_accessing(&self) -> ParseResult<Vec<String>> {
        let url = String::from("");
        let ret: Document;
        match reqwest::get(url).await {
            Err(_) => {
                return Err(error::new(
                    "parsing Error".to_string(),
                    ParsingErrorKind::NetworkError,
                ))
            }
            Ok(re) if (!re.status().is_success()) => {
                return Err(error::new(
                    "parsing Error".to_string(),
                    ParsingErrorKind::NetworkError,
                ))
            }
            Ok(re) => ret = Document::from_read(re.text().await.unwrap().as_bytes()).unwrap(),
        };
        let mut accessing_members: Vec<String> = Vec::new();
        for item in ret.find(Class("member_list")) {
            accessing_members.push(item.text().trim().to_string());
        }
        Ok(accessing_members)
    }
    pub fn get_clan_name(&self) -> &str {
        &self.name
    }
    pub fn get_clan_url(&self) -> &Option<String> {
        &self.post_url
    }
    pub fn open_clan_page(&self) -> bool {
        match &self.post_url {
            None => false,
            Some(url) => {
                webbrowser::open(&("clan link base ".to_string() + url)).expect("failed to open");
                //TODO: need to change link
                true
            }
        }
    }
    pub fn set_clan_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_clan_url(&mut self, post_url: String) {
        self.post_url = Some(post_url);
    }
}
