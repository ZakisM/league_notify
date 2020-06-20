#[derive(Debug)]
pub struct ChampionWinRate {
    champion_id: u32,
    champion_name: String,
    wins: u8,
    losses: u8,
    win_rate: u16,
    team_id: Option<u8>,
    rank: Option<String>,
}

impl ChampionWinRate {
    pub fn new(champion_id: u32, champion_name: String, wins: u8, losses: u8) -> Self {
        let w_f32 = wins as f32;
        let l_f32 = losses as f32;

        let win_rate = ((w_f32 / (w_f32 + l_f32)) * 100.0) as u16;

        ChampionWinRate {
            champion_id,
            champion_name,
            wins,
            losses,
            win_rate,
            team_id: None,
            rank: None,
        }
    }

    pub fn champion_name(&self) -> &str {
        &self.champion_name
    }

    pub fn win_rate_string(&self) -> String {
        format!("{}%", self.win_rate)
    }

    pub fn team_id(&self) -> u8 {
        match self.team_id {
            None => 1,
            Some(id) => id,
        }
    }

    pub fn set_team_id(&mut self, team_id: u8) {
        self.team_id = Some(team_id)
    }

    pub fn rank(&self) -> &str {
        match &self.rank {
            None => "Unranked",
            Some(r) => r,
        }
    }

    pub fn set_rank(&mut self, rank: String) {
        self.rank = Some(rank)
    }
}
