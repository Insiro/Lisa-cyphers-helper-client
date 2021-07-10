extern crate lisa;
mod private;

use reqwest;
use select::document::Document;
use select::predicate::Class;

#[tokio::test]
async fn get_accessing() {
    let url = private::clan_url();
    let res = reqwest::get(url).await.unwrap();
    assert!(res.status().is_success());
    let ret: Document = Document::from_read(res.text().await.unwrap().as_bytes()).unwrap();
    println!("accessing members : ");
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
