use super::Endpoint;

const SPECTATOR_ENDPOINT: &str = "lol/spectator/v4";

#[allow(unused)]
pub enum SpectatorEndpoint<'a> {
    BySummonerId(&'a str),
    FeaturedGames,
}

impl Endpoint for SpectatorEndpoint<'_> {
    fn url(self) -> String {
        match self {
            SpectatorEndpoint::BySummonerId(encrypted_summoner_id) => format!(
                "{}/active-games/by-summoner/{}",
                SPECTATOR_ENDPOINT, encrypted_summoner_id
            ),
            SpectatorEndpoint::FeaturedGames => format!("{}/featured-games", SPECTATOR_ENDPOINT),
        }
    }
}
