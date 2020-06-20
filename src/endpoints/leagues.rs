const LEAGUES_ENDPOINT: &str = "/lol/league/v4";

#[allow(unused)]
pub enum LeagueRankEndpoint<'a> {
    BySummonerId(&'a str),
}

impl LeagueRankEndpoint<'_> {
    pub fn url(&self) -> String {
        match self {
            LeagueRankEndpoint::BySummonerId(encrypted_summoner_id) => format!(
                "{}/entries/by-summoner/{}",
                LEAGUES_ENDPOINT, encrypted_summoner_id
            ),
        }
    }
}
