pub mod ddragon;
pub mod leagues;
pub mod lol_match;
pub mod spectator;
pub mod summoner;

pub trait Endpoint {
    fn url(self) -> String;
}
