use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorInfo {
    pub game_id: i64,
    pub map_id: i64,
    pub game_mode: String,
    pub game_type: String,
    pub game_queue_config_id: i64,
    pub participants: Vec<Participant>,
    pub observers: Observers,
    pub platform_id: String,
    pub banned_champions: Vec<BannedChampion>,
    pub game_start_time: i64,
    pub game_length: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub team_id: i64,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub champion_id: i64,
    pub profile_icon_id: i64,
    pub summoner_name: String,
    pub bot: bool,
    pub summoner_id: String,
    pub game_customization_objects: Vec<::serde_json::Value>,
    pub perks: Perks,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub perk_ids: Vec<i64>,
    pub perk_style: Option<i64>,
    pub perk_sub_style: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observers {
    pub encryption_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    pub champion_id: i64,
    pub team_id: i64,
    pub pick_turn: i64,
}
