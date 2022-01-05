use super::Endpoint;

const SUMMONER_ENDPOINT: &str = "lol/summoner/v4/summoners";

#[allow(unused)]
pub enum SummonerEndpointBy<'a> {
    Account(&'a str),
    Name(&'a str),
    Puuid(&'a str),
    SummonerId(&'a str),
}

impl Endpoint for SummonerEndpointBy<'_> {
    fn url(self) -> String {
        match self {
            SummonerEndpointBy::Account(encrypted_account_id) => {
                format!("{}/by-account/{}", SUMMONER_ENDPOINT, encrypted_account_id)
            }
            SummonerEndpointBy::Name(summoner_name) => {
                format!("{}/by-name/{}", SUMMONER_ENDPOINT, summoner_name)
            }
            SummonerEndpointBy::Puuid(encrypted_puuid) => {
                format!("{}/by-puuid/{}", SUMMONER_ENDPOINT, encrypted_puuid)
            }
            SummonerEndpointBy::SummonerId(encrypted_summoner_id) => {
                format!("{}/{}", SUMMONER_ENDPOINT, encrypted_summoner_id)
            }
        }
    }
}
