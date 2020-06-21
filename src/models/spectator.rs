use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct SpectatorInfo {
    game_id: i64,
    map_id: i64,
    game_mode: String,
    game_type: String,
    game_queue_config_id: i64,
    participants: Vec<Participant>,
    observers: Observers,
    platform_id: String,
    banned_champions: Vec<BannedChampion>,
    game_start_time: i64,
    game_length: i64,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    team_id: i64,
    spell1_id: i64,
    spell2_id: i64,
    champion_id: i64,
    profile_icon_id: i64,
    summoner_name: String,
    bot: bool,
    summoner_id: String,
    game_customization_objects: Vec<::serde_json::Value>,
    perks: Perks,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    perk_ids: Vec<i64>,
    perk_style: Option<i64>,
    perk_sub_style: Option<i64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Observers {
    encryption_key: String,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    champion_id: i64,
    team_id: i64,
    pick_turn: i64,
}
