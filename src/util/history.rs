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

pub mod new {
    use crate::page::Page;

    pub struct History {
        key: Option<String>,
        page: Page,
    }
    
    pub struct Histories(Vec<History>);
    impl Histories {
        pub fn new() -> Self {
            Histories(vec![])
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn insert(&mut self, history: History) {
            self.0.push(history);
        }
        pub fn pop(&mut self) -> Option<History> {
            self.0.pop()
        }
    }
}

pub struct History {
    page: Kind,
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
    pub fn new(page: Kind, key: Option<String>) -> History {
        History { page, key }
    }
}
