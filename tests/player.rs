extern crate lisa;
mod private;
use lisa::exs;
use lisa::object::clan;
use reqwest;
use select;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
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
        .expect("access err");
    let parsed: Value = serde_json::from_str(res.text().await.unwrap().as_str()).unwrap();
    let row = match parsed["rows"].get(0) {
        Some(row) => row,
        None => panic!(),
    };
    assert_eq!(row["playerId"].as_str().unwrap(), private::user::id());
    assert_eq!(row["nickname"].as_str().unwrap(), &test_name);
    assert_eq!(row["grade"], private::user::grade());
}
