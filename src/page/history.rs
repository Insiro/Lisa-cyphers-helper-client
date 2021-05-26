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
    name: Option<String>,
    page: Kind,
}

pub struct Histories {
    list: Vec<History>,
}

impl Histories {
    pub fn get_point(&self) -> String {
        let mut re = String::new();
        for item in self.list.iter() {
            re.push_str(item.get_page_name().as_str());
        }
        re
    }
    pub fn push(&mut self, history: History) {
        self.list.push(history);
    }
    pub fn init() -> Histories {
        Histories { list: vec![] }
    }
    pub fn change_name(&mut self, name: &str) {
        match self.list.last_mut() {
            None => {}
            Some(item) => item.name = Some(name.to_string()),
        }
    }
    pub fn get_curr(&self) -> Option<&History> {
        self.list.last()
    }
}

impl History {
    pub fn get_page_name(&self) -> String {
        match &self.page {
            Kind::Main => "> ".to_string(),
            Kind::Matches => "match > ".to_string(),
            _ => match &self.name {
                None => format!("> {}> ", self.page.as_str()),
                Some(name) => format!("> {} {}", self.page.as_str(), name),
            },
        }
    }
}
