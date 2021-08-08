use webbrowser;

use crate::error as lisa_error;
use getset::Getters;
use reqwest;
use select::document::Document;
use select::predicate::Class;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters)]
pub struct ClanBase {
    #[getset(get, set)]
    name: String,
    #[getset(get, set)]
    post_url: Option<String>,
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
    pub async fn get_accessing(&self) -> lisa_error::Result<Vec<String>> {
        let url = String::from("");
        let ret: Document;
        match reqwest::get(url).await {
            Err(_) => {
                return Err(lisa_error::new(
                    "parsing Error",
                    lisa_error::Kind::NetworkError,
                ))
            }
            Ok(re) if (!re.status().is_success()) => {
                return Err(lisa_error::new(
                    "parsing Error",
                    lisa_error::Kind::NetworkError,
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
}
