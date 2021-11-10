use crate::util::data_serializer::option_date_se;
use crate::util::UtcTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Kind {
    App,
    HotPotato,
    CyphersUpdate,
    Magagine,
    Event,
    CyphersNotic,
}

#[derive(Serialize, Deserialize)]
pub struct Notify {
    kind: Kind,
    link: Option<String>,
    title: Option<String>,
    #[serde(with = "option_date_se")]
    date: Option<UtcTime>,
}

//TODO: notify parser
#[allow(dead_code)]
impl Notify {
    fn dumy() -> Notify {
        Notify {
            kind: Kind::App,
            link: None,
            title: None,
            date: None,
        }
    }

    fn parse_app(_url: &str) -> Vec<Notify> {
        vec![Notify::dumy()]
    }
    fn update_cyphers() -> Vec<Notify> {
        vec![Notify::dumy()]
    }
    fn parse_cyphers(kind: Kind, _time: UtcTime) -> Vec<Notify> {
        let mut _url: String = match &kind {
            Kind::CyphersUpdate => "http://cyphers.nexon.com/cyphers/article/update/rss",
            Kind::Event => "http://cyphers.nexon.com/cyphers/article/event/rss",
            Kind::Magagine => "http://cyphers.nexon.com/cyphers/article/magazine/rss",
            Kind::CyphersNotic => "http://cyphers.nexon.com/cyphers/article/notice/rss",
            _ => return vec![],
        }
        .to_string();
        //TODO: parse description and
        vec![Notify::dumy()]
    }
}
