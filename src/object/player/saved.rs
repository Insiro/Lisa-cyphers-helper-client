#![allow(non_snake_case)]
use super::base::Base;
use super::{base, Gender};
use super::{Player, PlayerBuilder};
use crate::object::builder::Builder;
use crate::object::SavedObject;
use crate::object::{builder, Object};
use crate::util::data_serializer::option_date_se;
use crate::util::UtcTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Saved {
    player_info: Base,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    // position //TODO: put position
    #[serde(with = "option_date_se")]
    birth_day: Option<UtcTime>,
}

impl Player for Saved {
    fn get_id(&self) -> &str {
        &self.player_info.get_id()
    }
    fn get_name(&self) -> &str {
        &self.player_info.get_name()
    }
    fn get_grade(&self) -> u8 {
        self.player_info.grade
    }
}

impl SavedObject for Saved {
    fn save() {
        todo!()
    }

    fn load() {
        todo!()
    }
}

impl Object for Saved {}

impl Saved {
    fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }
    fn get_gender(&self) -> &Gender {
        &self.gender
    }
    fn set_birthday(&mut self, birhday: Option<UtcTime>) {
        self.birth_day = birhday;
    }
    fn get_birthday(&self) -> &Option<UtcTime> {
        &self.birth_day
    }
}
// trait PlayerSavedData {
//     fn set_name(&mut self);
//     fn get_name(&self) -> &Option<String>;
// }

pub struct SavedBuilder {
    player: Option<Saved>,
}

impl builder::Builder for SavedBuilder {
    type Target = Saved;
    fn new() -> Self {
        Self { player: None }
    }
    fn build(&mut self) -> Result<Self::Target, Box<dyn std::error::Error>> {
        match &self.player {
            Some(_) => Ok(self.player.take().unwrap()),
            None => Err(Box::new(builder::Error::new(
                "ID not Initialized".to_string(),
            ))),
        }
    }
}

impl PlayerBuilder for SavedBuilder {
    fn set_id(&mut self, id: &str) {
        let mut baseBuilder = base::BaseBuilder::new();
        baseBuilder.set_id(id);
        let base = baseBuilder.build().expect("Failed to load player");
        self.player = Some(Saved {
            player_info: base,
            memo: None,
            gender: Gender::Unknown,
            name: None,
            birth_day: None,
        });
    }
}

impl SavedBuilder {
    pub fn from_player(player: Box<&dyn Player>) -> Self {
        let base = Base::from_player(player);

        Self {
            player: Self::from_base(base).player.take(),
        }
    }
    pub fn from_base(player_base: base::Base) -> Self {
        Self {
            player: Some(Saved {
                memo: None,
                gender: Gender::Unknown,
                name: None,
                birth_day: None,
                player_info: player_base,
            }),
        }
    }
    pub fn memo(&mut self, memo: Option<String>) {
        match &mut self.player {
            Some(player) => player.memo = memo,
            None => {
                panic!("you must set id first!")
            }
        };
    }
    pub fn gender(&mut self, gender: Gender) {
        match &mut self.player {
            Some(player) => player.gender = gender,
            None => {
                panic!("you must set id first!")
            }
        };
    }
    pub fn set_player_name(&mut self, name: Option<String>) {
        match &mut self.player {
            Some(player) => player.name = name,
            None => {
                panic!("you must set id first!")
            }
        };
    }
    pub fn set_birthday(&mut self, birth_day: Option<UtcTime>) {
        match &mut self.player {
            Some(player) => player.birth_day = birth_day,
            None => {
                panic!("you must set id first!")
            }
        };
    }
}
