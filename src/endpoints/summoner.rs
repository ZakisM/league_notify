const SUMMONER_ENDPOINT: &str = "/lol/summoner/v4/summoners";

#[allow(unused)]
pub enum SummonerEndpoint<'a> {
    ByAccount(&'a str),
    ByName(&'a str),
    ByPuuid(&'a str),
    BySummonerId(&'a str),
}

impl SummonerEndpoint<'_> {
    pub fn url(&self) -> String {
        match self {
            SummonerEndpoint::ByAccount(encrypted_account_id) => {
                format!("{}/by-account/{}", SUMMONER_ENDPOINT, encrypted_account_id)
            }
            SummonerEndpoint::ByName(summoner_name) => {
                format!("{}/by-name/{}", SUMMONER_ENDPOINT, summoner_name)
            }
            SummonerEndpoint::ByPuuid(encrypted_puuid) => {
                format!("{}/by-puuid/{}", SUMMONER_ENDPOINT, encrypted_puuid)
            }
            SummonerEndpoint::BySummonerId(encrypted_summoner_id) => {
                format!("{}/{}", SUMMONER_ENDPOINT, encrypted_summoner_id)
            }
        }
    }
}
