use super::PageTrait;

pub fn help(_args: Vec<String>) {}
pub struct Set {}

impl PageTrait for Set {
    fn load(_key: String) -> Self {
        todo!()
    }

    fn get_key(&self) -> String {
        todo!()
    }
}
