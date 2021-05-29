extern crate lisa;
mod private;
use json;
use lisa::exs;
use lisa::object::clan;
use reqwest;
use select;
pub struct ParsedPlayer {
    player_name: String,
    player_id: Option<String>,
    win_count: u8,
    lose_count: u8,
    tier: Option<String>,
    tier_point: u8,
    clan: Option<clan::ClanBase>,
}
#[tokio::test]
async fn get_info() {
    let test_name = private::user::name();
    let client = reqwest::Client::new();
    let res = client
        .get(exs::cy_api_url() + "players/")
        .query(&[
            ("apikey", private::neople_key().as_str()),
            ("nickname", &test_name),
        ])
        .send()
        .await
        .unwrap();
    let parsed = json::parse(res.text().await.unwrap().as_str()).unwrap();
    let row = parsed["rows"][0].clone();
    println!("parsed Data\n{}", json::stringify(parsed.clone()));
    assert_eq!(row["playerId"].as_str().unwrap(), private::user::id());
    assert_eq!(row["nickname"].as_str().unwrap(), &test_name);
    assert_eq!(row["grade"].as_u8().unwrap(), private::user::grade());
}
#[tokio::test]
async fn parsed_payer(){
    
}