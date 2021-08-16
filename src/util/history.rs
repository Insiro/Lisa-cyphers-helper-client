use crate::command::Command;
use crate::util::history;

pub enum Kind {
    Main,
    Matches,
    Profile,
    GameRecord,
    Notic,
}
impl Kind {
    pub fn as_str(&self) -> &str {
        match self {
            Kind::Main => "main",
            Kind::Matches => "match",
            Kind::Profile => "profile",
            Kind::GameRecord => "record",
            Kind::Notic => "notic",
        }
    }
}
pub struct History {
    page: history::Kind,
    key: Option<String>,
}
pub fn init() -> Histories {
    Histories { list: vec![] }
}
pub struct Histories {
    list: Vec<History>,
}
impl Histories {
    pub fn inset(&mut self) {}
    pub fn clear(&mut self) {
        self.list.clear();
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn cli_point(&self) -> String {
        let mut re = String::new();
        for item in self.list.iter() {
            re.push_str(item.cli_page_name().as_str());
        }
        re
    }
    pub fn push(&mut self, history: History) {
        self.list.push(history);
    }

    pub fn change_key(&mut self, key: &str) {
        match self.list.last_mut() {
            None => {}
            Some(item) => item.key = Some(key.to_string()),
        }
    }
    pub fn get_curr(&self) -> Option<&History> {
        self.list.last()
    }
    pub fn new(page: history::Kind, key: Option<String>) -> History {
        History { page, key }
    }
    pub fn run_cli(&mut self) -> Command {
        //run last page as cli
        Command::NotImpletated
    }
}

impl History {
    pub fn cli_page_name(&self) -> String {
        match &self.page {
            Kind::Main => "> ".to_string(),
            Kind::Matches => "match > ".to_string(),
            _ => match &self.key {
                None => format!("> {}> ", self.page.as_str()),
                Some(key) => format!("> {} {}", self.page.as_str(), key),
            },
        }
    }
}
