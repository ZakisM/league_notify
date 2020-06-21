use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct LeagueRank {
    league_id: String,
    queue_type: String,
    tier: String,
    rank: String,
    summoner_id: String,
    summoner_name: String,
    league_points: i64,
    wins: i64,
    losses: i64,
    veteran: bool,
    inactive: bool,
    fresh_blood: bool,
    hot_streak: bool,
}
