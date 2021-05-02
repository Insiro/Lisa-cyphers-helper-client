use chrono::{DateTime, Utc};

type UtcTime = Option<DateTime<Utc>>;

enum Kind {
    App,
    HotPotato,
    CyphersUpdate,
    Magagine,
    Event,
    CyphersNotic,
}
pub struct Notify {
    kind: Kind,
    link: Option<String>,
    title: Option<String>,
    date: UtcTime,
}

impl Notify {
    fn dumy() -> Notify {
        Notify {
            kind: Kind::App,
            link: None,
            title: None,
            date: None,
        }
    }
    fn parse_app(url: &str) -> Vec<Notify> {
        vec![Notify::dumy()]
    }
    fn update_cyphers() -> Vec<Notify> {
        vec![Notify::dumy()]
    }
    fn parse_cyphers(kind: Kind, time: UtcTime) -> Vec<Notify> {
        let mut url: &str;
        url = match &kind {
            Kind::CyphersUpdate => "http://cyphers.nexon.com/cyphers/article/update/rss",
            Kind::Event => "http://cyphers.nexon.com/cyphers/article/event/rss",
            Kind::Magagine => "http://cyphers.nexon.com/cyphers/article/magazine/rss",
            Kind::CyphersNotic => "http://cyphers.nexon.com/cyphers/article/notice/rss",
            _ => return vec![],
        };
        //TODO: parse description and
        vec![Notify::dumy()]
    }
}
