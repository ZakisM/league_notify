use std::cmp::Ordering;

use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ChampionWinRate {
    champion_id: u64,
    champion_name: String,
    wins: u8,
    losses: u8,
    total_games: u8,
    win_rate: u16,
    team_id: u8,
    rank: String,
    summoner_name: String,
}

impl std::cmp::Ord for ChampionWinRate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.team_id
            .cmp(&other.team_id)
            .then(self.win_rate.cmp(&other.win_rate).reverse())
    }
}

impl std::cmp::PartialOrd for ChampionWinRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for ChampionWinRate {}

impl std::cmp::PartialEq for ChampionWinRate {
    fn eq(&self, other: &Self) -> bool {
        self.champion_id == other.champion_id
    }
}

impl ChampionWinRate {
    pub fn new(champion_id: u64, champion_name: String, wins: u8, losses: u8) -> Self {
        let w_f32 = wins as f32;
        let l_f32 = losses as f32;

        let win_rate = ((w_f32 / (w_f32 + l_f32)) * 100.0) as u16;

        ChampionWinRate {
            champion_id,
            champion_name,
            wins,
            losses,
            win_rate,
            total_games: wins + losses,
            team_id: 0,
            rank: "Unranked".to_owned(),
            summoner_name: "Unknown".to_owned(),
        }
    }

    pub fn win_rate_string(&self) -> String {
        if self.total_games > 0 {
            format!(
                "{}% in last {} {}.",
                self.win_rate,
                self.total_games,
                match self.total_games {
                    1 => "game",
                    _ => "games",
                }
            )
        } else {
            "No games played.".to_owned()
        }
    }
}
