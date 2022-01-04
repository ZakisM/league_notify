use std::fmt::Write;

use serde::Serialize;

const MATCH_ENDPOINT: &str = "lol/match/v5/matches";

#[allow(unused)]
pub enum MatchEndpoint<'a> {
    ByPuuid(&'a str, Option<ByPuiidParams>),
    ByMatchId(&'a str),
    TimelineByMatchId(&'a str),
}

#[derive(Builder, Default, Serialize)]
#[builder(setter(strip_option), default)]
#[serde(rename_all = "camelCase")]
pub struct ByPuiidParams {
    start_time: Option<i64>,
    end_time: Option<i64>,
    queue: Option<u8>,
    r#type: Option<String>,
    start: Option<u8>,
    count: Option<u8>,
}

impl MatchEndpoint<'_> {
    pub fn url(self) -> String {
        match self {
            MatchEndpoint::ByPuuid(puuid, params) => {
                let mut res = format!("{}/by-puuid/{}/ids", MATCH_ENDPOINT, puuid);

                if let Some(params) = params.and_then(|p| serde_url_params::to_string(&p).ok()) {
                    write!(res, "?{}", params)
                        .expect("Failed to generate MatchEndpoint::ByPuuid with params");
                }

                res
            }
            MatchEndpoint::ByMatchId(match_id) => {
                format!("{}/{}", MATCH_ENDPOINT, match_id)
            }
            MatchEndpoint::TimelineByMatchId(match_id) => {
                format!("{}/{}/timeline", MATCH_ENDPOINT, match_id)
            }
        }
    }
}
