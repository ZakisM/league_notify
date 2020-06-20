use core::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::api::Api;
use crate::endpoints::lol_match;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub matches: Vec<Match>,
    pub start_index: i64,
    pub end_index: i64,
    pub total_games: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub platform_id: String,
    pub game_id: i64,
    pub champion: i64,
    pub queue: i64,
    pub season: i64,
    pub timestamp: i64,
    pub role: String,
    pub lane: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchData {
    pub game_id: i64,
    pub platform_id: String,
    pub game_creation: i64,
    pub game_duration: i64,
    pub queue_id: i64,
    pub map_id: i64,
    pub season_id: i64,
    pub game_version: String,
    pub game_mode: String,
    pub game_type: String,
    pub teams: Vec<Team>,
    pub participants: Vec<Participant>,
    pub participant_identities: Vec<ParticipantIdentity>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_id: i64,
    pub win: String,
    pub first_blood: bool,
    pub first_tower: bool,
    pub first_inhibitor: bool,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub first_rift_herald: bool,
    pub tower_kills: i64,
    pub inhibitor_kills: i64,
    pub baron_kills: i64,
    pub dragon_kills: i64,
    pub vilemaw_kills: i64,
    pub rift_herald_kills: i64,
    pub dominion_victory_score: i64,
    pub bans: Vec<::serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i64,
    pub team_id: i64,
    pub champion_id: i64,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub highest_achieved_season_tier: Option<String>,
    pub stats: Stats,
    pub timeline: Timeline,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub participant_id: i64,
    pub win: bool,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub killing_sprees: i64,
    pub longest_time_spent_living: i64,
    pub double_kills: i64,
    pub triple_kills: i64,
    pub quadra_kills: i64,
    pub penta_kills: i64,
    pub unreal_kills: i64,
    pub total_damage_dealt: i64,
    pub magic_damage_dealt: i64,
    pub physical_damage_dealt: i64,
    pub true_damage_dealt: i64,
    pub largest_critical_strike: i64,
    pub total_damage_dealt_to_champions: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub true_damage_dealt_to_champions: i64,
    pub total_heal: i64,
    pub total_units_healed: i64,
    pub damage_self_mitigated: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub vision_score: i64,
    #[serde(rename = "timeCCingOthers")]
    pub time_ccing_others: i64,
    pub total_damage_taken: i64,
    pub magical_damage_taken: i64,
    pub physical_damage_taken: i64,
    pub true_damage_taken: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub turret_kills: i64,
    pub inhibitor_kills: i64,
    pub total_minions_killed: i64,
    pub neutral_minions_killed: Option<i64>,
    pub neutral_minions_killed_team_jungle: Option<i64>,
    pub neutral_minions_killed_enemy_jungle: Option<i64>,
    pub total_time_crowd_control_dealt: i64,
    pub champ_level: i64,
    pub vision_wards_bought_in_game: i64,
    pub sight_wards_bought_in_game: i64,
    pub wards_placed: Option<i64>,
    pub wards_killed: Option<i64>,
    pub first_blood_kill: Option<bool>,
    pub first_blood_assist: Option<bool>,
    pub first_tower_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_inhibitor_kill: Option<bool>,
    pub first_inhibitor_assist: Option<bool>,
    pub combat_player_score: i64,
    pub objective_player_score: i64,
    pub total_player_score: i64,
    pub total_score_rank: i64,
    pub player_score0: i64,
    pub player_score1: i64,
    pub player_score2: i64,
    pub player_score3: i64,
    pub player_score4: i64,
    pub player_score5: i64,
    pub player_score6: i64,
    pub player_score7: i64,
    pub player_score8: i64,
    pub player_score9: i64,
    pub perk0: i64,
    pub perk0_var1: i64,
    pub perk0_var2: i64,
    pub perk0_var3: i64,
    pub perk1: i64,
    pub perk1_var1: i64,
    pub perk1_var2: i64,
    pub perk1_var3: i64,
    pub perk2: i64,
    pub perk2_var1: i64,
    pub perk2_var2: i64,
    pub perk2_var3: i64,
    pub perk3: i64,
    pub perk3_var1: i64,
    pub perk3_var2: i64,
    pub perk3_var3: i64,
    pub perk4: i64,
    pub perk4_var1: i64,
    pub perk4_var2: i64,
    pub perk4_var3: i64,
    pub perk5: i64,
    pub perk5_var1: i64,
    pub perk5_var2: i64,
    pub perk5_var3: i64,
    pub perk_primary_style: Option<i64>,
    pub perk_sub_style: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub participant_id: i64,
    pub creeps_per_min_deltas: Option<CreepsPerMinDeltas>,
    pub xp_per_min_deltas: Option<XpPerMinDeltas>,
    pub gold_per_min_deltas: Option<GoldPerMinDeltas>,
    pub damage_taken_per_min_deltas: Option<DamageTakenPerMinDeltas>,
    pub role: String,
    pub lane: String,
    pub cs_diff_per_min_deltas: Option<CsDiffPerMinDeltas>,
    pub xp_diff_per_min_deltas: Option<XpDiffPerMinDeltas>,
    pub damage_taken_diff_per_min_deltas: Option<DamageTakenDiffPerMinDeltas>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreepsPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XpPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoldPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageTakenPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XpDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageTakenDiffPerMinDeltas {
    #[serde(rename = "10-20")]
    pub n1020: Option<f64>,
    #[serde(rename = "0-10")]
    pub n010: Option<f64>,
    #[serde(rename = "30-end")]
    pub n30_end: Option<f64>,
    #[serde(rename = "20-30")]
    pub n2030: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    pub participant_id: i64,
    pub player: Player,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub platform_id: String,
    pub account_id: String,
    pub summoner_name: String,
    pub summoner_id: String,
    pub current_platform_id: String,
    pub current_account_id: String,
    pub match_history_uri: String,
    pub profile_icon: i64,
}

#[derive(Debug)]
pub struct LeagueMatchList<'a> {
    pub match_info: LeagueMatchInfo<'a>,
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

#[derive(Debug)]
pub struct LeagueMatchInfo<'a> {
    pub matches: Vec<LeagueMatch<'a>>,
    pub start_index: i64,
    pub end_index: i64,
    pub total_games: i64,
}

#[derive(Debug)]
pub struct LeagueMatch<'a> {
    pub mat: Match,
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
