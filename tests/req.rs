use reqwest;
use select::document::Document;
use select::predicate::Class;
use tokio::{self};
// use std::future::Future;

#[tokio::test]
async fn t1() {
    //TODO: controll unwrap
    let res = reqwest::get("http://cyphers.nexon.com/cyphers/clan/zealot")
        .await
        .unwrap();
    assert!(res.status().is_success());
    let ret = Document::from_read(res.text().await.unwrap().as_bytes()).unwrap();
    let mut accessing_members: Vec<String> = Vec::new();
    for item in ret.find(Class("member_list")) {
        let mut name = String::new();
        for c in item.text().trim().chars() {
            if !c.is_whitespace() {
                name.push(c);
            }
        }
        accessing_members.push(name);
    }
    println!("body = {:#?}", accessing_members);
}
