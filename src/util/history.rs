use crate::page::Page;

pub struct History {
    page: Page,
    key: Option<String>,
}

impl History {
    fn load(&self) {
        let ui = self.page.get_ui();
        let page = self.page.load();

        todo!()
    }
    pub fn new(page: Page, key: Option<String>) -> Self {
        Self { page, key }
    }
}

pub fn init() -> Histories {
    Histories(vec![])
}

pub struct Histories(Vec<History>);

impl Histories {
    pub fn insert(&mut self, history: History) {
        self.0.push(history);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<History> {
        self.0.pop()
    }

    pub fn push(&mut self, history: History) {
        self.0.push(history);
    }

    pub fn change_key(&mut self, key: &str) {
        match self.0.last_mut() {
            None => {}
            Some(item) => item.key = Some(key.to_string()),
        }
    }
    pub fn get_curr(&self) -> Option<&History> {
        self.0.last()
    }
}
