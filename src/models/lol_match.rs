use core::fmt;
use std::fmt::Formatter;

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::api::Api;
use crate::endpoints::lol_match;
use crate::Result;

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    matches: Vec<Match>,
    start_index: i64,
    end_index: i64,
    total_games: i64,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Match {
    platform_id: String,
    game_id: i64,
    champion: i64,
    queue: i64,
    season: i64,
    timestamp: i64,
    role: String,
    lane: String,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct MatchData {
    game_id: i64,
    platform_id: String,
    game_creation: i64,
    game_duration: i64,
    queue_id: i64,
    map_id: i64,
    season_id: i64,
    game_version: String,
    game_mode: String,
    game_type: String,
    teams: Vec<Team>,
    participants: Vec<Participant>,
    participant_identities: Vec<ParticipantIdentity>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Team {
    team_id: i64,
    win: String,
    first_blood: bool,
    first_tower: bool,
    first_inhibitor: bool,
    first_baron: bool,
    first_dragon: bool,
    first_rift_herald: bool,
    tower_kills: i64,
    inhibitor_kills: i64,
    baron_kills: i64,
    dragon_kills: i64,
    vilemaw_kills: i64,
    rift_herald_kills: i64,
    dominion_victory_score: i64,
    bans: Vec<::serde_json::Value>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    participant_id: i64,
    team_id: i64,
    champion_id: u64,
    spell1_id: i64,
    spell2_id: i64,
    highest_achieved_season_tier: Option<String>,
    stats: Stats,
    timeline: Timeline,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    participant_id: i64,
    win: bool,
    item0: i64,
    item1: i64,
    item2: i64,
    item3: i64,
    item4: i64,
    item5: i64,
    item6: i64,
    kills: i64,
    deaths: i64,
    assists: i64,
    largest_killing_spree: i64,
    largest_multi_kill: i64,
    killing_sprees: i64,
    longest_time_spent_living: i64,
    double_kills: i64,
    triple_kills: i64,
    quadra_kills: i64,
    penta_kills: i64,
    unreal_kills: i64,
    total_damage_dealt: i64,
    magic_damage_dealt: i64,
    physical_damage_dealt: i64,
    true_damage_dealt: i64,
    largest_critical_strike: i64,
    total_damage_dealt_to_champions: i64,
    magic_damage_dealt_to_champions: i64,
    physical_damage_dealt_to_champions: i64,
    true_damage_dealt_to_champions: i64,
    total_heal: i64,
    total_units_healed: i64,
    damage_self_mitigated: i64,
    damage_dealt_to_objectives: i64,
    damage_dealt_to_turrets: i64,
    vision_score: i64,
    #[serde(rename = "timeCCingOthers")]
    time_ccing_others: i64,
    total_damage_taken: i64,
    magical_damage_taken: i64,
    physical_damage_taken: i64,
    true_damage_taken: i64,
    gold_earned: i64,
    gold_spent: i64,
    turret_kills: i64,
    inhibitor_kills: i64,
    total_minions_killed: i64,
    neutral_minions_killed: Option<i64>,
    neutral_minions_killed_team_jungle: Option<i64>,
    neutral_minions_killed_enemy_jungle: Option<i64>,
    total_time_crowd_control_dealt: i64,
    champ_level: i64,
    vision_wards_bought_in_game: i64,
    sight_wards_bought_in_game: i64,
    wards_placed: Option<i64>,
    wards_killed: Option<i64>,
    first_blood_kill: Option<bool>,
    first_blood_assist: Option<bool>,
    first_tower_kill: Option<bool>,
    first_tower_assist: Option<bool>,
    first_inhibitor_kill: Option<bool>,
    first_inhibitor_assist: Option<bool>,
    combat_player_score: i64,
    objective_player_score: i64,
    total_player_score: i64,
    total_score_rank: i64,
    player_score0: i64,
    player_score1: i64,
    player_score2: i64,
    player_score3: i64,
    player_score4: i64,
    player_score5: i64,
    player_score6: i64,
    player_score7: i64,
    player_score8: i64,
    player_score9: i64,
    perk0: i64,
    perk0_var1: i64,
    perk0_var2: i64,
    perk0_var3: i64,
    perk1: i64,
    perk1_var1: i64,
    perk1_var2: i64,
    perk1_var3: i64,
    perk2: i64,
    perk2_var1: i64,
    perk2_var2: i64,
    perk2_var3: i64,
    perk3: i64,
    perk3_var1: i64,
    perk3_var2: i64,
    perk3_var3: i64,
    perk4: i64,
    perk4_var1: i64,
    perk4_var2: i64,
    perk4_var3: i64,
    perk5: i64,
    perk5_var1: i64,
    perk5_var2: i64,
    perk5_var3: i64,
    perk_primary_style: Option<i64>,
    perk_sub_style: Option<i64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    participant_id: i64,
    creeps_per_min_deltas: Option<CreepsPerMinDeltas>,
    xp_per_min_deltas: Option<XpPerMinDeltas>,
    gold_per_min_deltas: Option<GoldPerMinDeltas>,
    damage_taken_per_min_deltas: Option<DamageTakenPerMinDeltas>,
    role: String,
    lane: String,
    cs_diff_per_min_deltas: Option<CsDiffPerMinDeltas>,
    xp_diff_per_min_deltas: Option<XpDiffPerMinDeltas>,
    damage_taken_diff_per_min_deltas: Option<DamageTakenDiffPerMinDeltas>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct CreepsPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct XpPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct GoldPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct DamageTakenPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct CsDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct XpDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct DamageTakenDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    n1020: Option<f64>,
    #[serde(rename = "0-10")]
    n010: Option<f64>,
    #[serde(rename = "30-end")]
    n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    n2030: Option<f64>,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    participant_id: i64,
    player: Player,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Player {
    platform_id: String,
    account_id: String,
    summoner_name: String,
    summoner_id: String,
    current_platform_id: String,
    current_account_id: String,
    match_history_uri: String,
    profile_icon: i64,
}

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct LeagueMatchList<'a> {
    match_info: LeagueMatchInfo<'a>,
    api: &'a Api<'a>,
}

impl fmt::Display for LeagueMatchList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.match_info)
    }
}

impl<'a> LeagueMatchList<'a> {
    pub fn new(match_info: MatchInfo, api: &'a Api<'_>) -> Self {
        let matches = match_info
            .matches
            .into_iter()
            .map(|minfo| LeagueMatch::new(minfo, api))
            .collect();

        Self {
            match_info: LeagueMatchInfo {
                matches,
                start_index: match_info.start_index,
                end_index: match_info.end_index,
                total_games: match_info.total_games,
            },
            api,
        }
    }
}

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct LeagueMatchInfo<'a> {
    matches: Vec<LeagueMatch<'a>>,
    start_index: i64,
    end_index: i64,
    total_games: i64,
}

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct LeagueMatch<'a> {
    mat: Match,
    api: &'a Api<'a>,
}

impl fmt::Display for LeagueMatch<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.mat)
    }
}

impl<'a> LeagueMatch<'a> {
    pub fn new(mat: Match, api: &'a Api<'a>) -> Self {
        Self { mat, api }
    }

    pub async fn match_data(&self) -> Result<MatchData> {
        self.api
            .get_match(lol_match::MatchEndpoint::ByMatchId(&self.mat.game_id))
            .await
    }
}
