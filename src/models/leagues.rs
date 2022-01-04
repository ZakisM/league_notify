use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueRank {
    pub league_id: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    pub veteran: bool,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub hot_streak: bool,
}
