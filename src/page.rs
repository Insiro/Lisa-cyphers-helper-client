pub mod matches;
pub mod profile;
pub mod record;
pub mod set;
use crate::gui::UI;
pub enum Page {
    Recrod(String),
    Matches(String),
    Set(String),
}
impl Page {
    pub fn load(&self) -> Box<dyn PageTrait> {
        match self {
            Page::Recrod(key) => Box::new(record::Record::load(key.to_string())),
            Page::Matches(key) => Box::new(matches::MatchInfo::load(key.to_string())),
            Page::Set(key) => Box::new(set::Set::load(key.to_string())),
        }
    }
    pub fn get_ui(&self) -> UI {
        match self {
            Page::Recrod(_) => UI::Records,
            Page::Matches(_) => UI::Matches,
            Page::Set(_) => UI::Settings,
        }
    }
}

pub trait PageTrait {
    fn load(key: String) -> Self
    where
        Self: Sized;
    fn get_key(&self) -> String;
}
