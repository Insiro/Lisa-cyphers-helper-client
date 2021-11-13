#![allow(non_snake_case)]
use super::{base, Gender};
use super::{Player, PlayerBuilder};
use crate::object::Object;
use crate::object::SavedObject;
use crate::util::data_serializer::option_date_se;
use crate::util::UtcTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Saved {
    player_info: base::Base,
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
        &self.get_birthday()
    }
}
// trait PlayerSavedData {
//     fn set_name(&mut self);
//     fn get_name(&self) -> &Option<String>;
// }

pub struct SavedBuilder {
    info: base::Base,
}

impl PlayerBuilder for SavedBuilder {
    type Player = Saved;
    fn new(id: String) -> Result<Self, Box<(dyn std::error::Error)>> {
        let builder = base::BaseBuilder::new(id)?;
        let info = builder.build()?;
        Ok(Self { info })
    }

    fn build(&self) -> Result<Self::Player, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl SavedBuilder {
    pub fn from_player(player: &Box<dyn Player>) -> Self {
        let p = player.as_ref();
        let base = base::Base::from_player(player);
        Self::from_base(base)
    }
    pub fn from_base(player_base: base::Base) -> Self {
        Self { info: player_base }
    }
}
