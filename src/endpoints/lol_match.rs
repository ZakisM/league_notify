use serde::Serialize;

const MATCH_ENDPOINT: &str = "/lol/match/v4";

#[allow(unused)]
pub enum MatchEndpoint<'a> {
    ByMatchId(&'a i64),
    ByAccountId(&'a str, Option<ByAccountIdParams>),
    TimelineByMatchId(&'a str),
    ByTournamentCode(&'a str),
    ByMatchIdTournamentCode(&'a str, &'a str),
}

#[derive(Builder, Default, Serialize)]
#[builder(setter(strip_option), default)]
#[serde(rename_all = "camelCase")]
pub struct ByAccountIdParams {
    champion: Option<u32>,
    queue: Option<u8>,
    season: Option<u8>,
    end_time: Option<u64>,
    begin_time: Option<u64>,
    end_index: Option<u16>,
    start_index: Option<u16>,
}

impl MatchEndpoint<'_> {
    pub fn url(&self) -> String {
        match self {
            MatchEndpoint::ByMatchId(match_id) => {
                format!("{}/matches/{}", MATCH_ENDPOINT, match_id)
            }
            MatchEndpoint::ByAccountId(encrypted_account_id, params) => format!(
                "{}/matchlists/by-account/{}{}",
                MATCH_ENDPOINT,
                encrypted_account_id,
                match params {
                    None => "".to_owned(),
                    Some(p) => format!(
                        "?{}",
                        &serde_url_params::to_string(&p).expect("Invalid params")
                    ),
                }
            ),
            MatchEndpoint::TimelineByMatchId(match_id) => {
                format!("{}/timelines/by-match/{}", MATCH_ENDPOINT, match_id)
            }
            MatchEndpoint::ByTournamentCode(tournament_code) => format!(
                "{}/matches/by-tournament-code/{}/ids",
                MATCH_ENDPOINT, tournament_code
            ),
            MatchEndpoint::ByMatchIdTournamentCode(match_id, tournament_code) => format!(
                "{}/matches/{}/by-tournament-code/{}",
                MATCH_ENDPOINT, match_id, tournament_code
            ),
        }
    }
}
