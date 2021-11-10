
use super::*;
#[test]
fn generate_client() {
    Client::new();
}

#[test]
fn rw_lock_client() {
    let ins2: RwLockCLient = Client::instance();
    let borrowed = ins2.borrow_mut();
    let mut instance = borrowed.try_write().unwrap();
    println!("version: {}", instance.meta.get_version());
    (*instance).setting.neople_api_key = "test".to_string();
    instance.subscription.save().expect("failed to save");
}
