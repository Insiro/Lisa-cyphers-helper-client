use ux::u4;
pub struct Charactor {
    name: String,
    position: u4, //Bit mask as (Tanker, Long, Close, Support)
    prev_img: Option<String>,
}
pub type CharList = Vec<Charactor>;
impl Charactor {
    pub fn dumy() -> Charactor {
        Charactor {
            name: "asdasd".to_string(),
            position: u4::new(0),
            prev_img: None,
        }
    }
    pub fn new(name: String, prev_img: Option<String>) -> Charactor {
        Charactor {
            name,
            prev_img,
            position: u4::new(0b0000),
        }
    }
    pub fn is_tanker(&self) -> bool {
        u8::from(self.position & u4::new(0b1000)) > 0
    }
    pub fn is_suppoerter(&self) -> bool {
        u8::from(self.position & u4::new(0b0001)) > 0
    }
    pub fn is_long_ranger(&self) -> bool {
        u8::from(self.position & u4::new(0b0100)) > 0
    }
    pub fn is_close_ranger(&self) -> bool {
        u8::from(self.position & u4::new(0b0010)) > 0
    }
    pub fn is_all_position(&self) -> bool {
        self.position == u4::new(0b1111)
    }
    pub fn mask_position(&self, mask: u4) -> bool {
        u8::from(self.position & mask) > 0
    }
    pub fn get_position_bit(&self) -> &u4 {
        &self.position
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_img_url(&self) -> Option<&str> {
        match &self.prev_img {
            Some(string) => Some(&string),
            None => None,
        }
    }
    pub fn set_img_url(&mut self, url: Option<&str>) {
        match url {
            None => self.prev_img = None,
            Some(url) => self.prev_img = Some(url.to_string()),
        }
    }
    pub fn set_position(&mut self, position_bit: u4) {
        self.position = position_bit;
    }
}