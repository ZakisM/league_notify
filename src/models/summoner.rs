use std::fmt;
use std::fmt::Formatter;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::api::Api;
use crate::endpoints::lol_match::{ByPuiidParams, ByPuiidParamsBuilder};
use crate::endpoints::{leagues, lol_match, spectator, summoner};
use crate::models::champion::ChampionWinRate;
use crate::models::leagues::LeagueRank;
use crate::models::lol_match::LeagueMatchList;
use crate::models::spectator::SpectatorInfo;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: i64,
    pub revision_date: i64,
    pub summoner_level: i64,
}

#[derive(Debug)]
pub struct SummonerCurrentGameInfo<'a> {
    pub summoner: Summoner<'a>,
    pub champion_id: i64,
    pub team_id: u8,
}

impl<'a> SummonerCurrentGameInfo<'a> {
    pub fn new(summoner: Summoner<'a>, champion_id: i64, team_id: u8) -> Self {
        SummonerCurrentGameInfo {
            summoner,
            champion_id,
            team_id,
        }
    }
}

#[derive(Debug)]
pub struct CurrentGameInfo<'a> {
    pub game_id: u64,
    pub summoners: Vec<SummonerCurrentGameInfo<'a>>,
}

impl<'a> CurrentGameInfo<'a> {
    pub fn new(game_id: u64, summoners: Vec<SummonerCurrentGameInfo<'a>>) -> Self {
        CurrentGameInfo { game_id, summoners }
    }
}

pub struct Summoner<'a> {
    pub summoner_info: SummonerInfo,
    api: &'a Api<'a>,
}

impl std::cmp::Eq for Summoner<'_> {}

impl std::cmp::PartialEq for Summoner<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.summoner_info.id == other.summoner_info.id
    }
}

impl std::hash::Hash for Summoner<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.summoner_info.id.hash(state)
    }
}

impl fmt::Debug for Summoner<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.summoner_info)
    }
}

impl fmt::Display for Summoner<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.summoner_info)
    }
}

impl<'a> Summoner<'a> {
    pub fn new(summoner_info: SummonerInfo, api: &'a Api<'a>) -> Self {
        Self { summoner_info, api }
    }

    pub async fn spectator(&self) -> Result<SpectatorInfo> {
        self.api
            .get_spectator(spectator::SpectatorEndpoint::BySummonerId(
                &self.summoner_info.id,
            ))
            .await
    }

    pub async fn solo_queue_rank(&self) -> Result<LeagueRank> {
        let league_ranks: Vec<LeagueRank> = self
            .api
            .get_leagues(leagues::LeagueRankEndpoint::BySummonerId(
                &self.summoner_info.id,
            ))
            .await?;

        match league_ranks
            .into_iter()
            .find(|l| l.queue_type == "RANKED_SOLO_5x5")
        {
            None => Err(anyhow!("Could not find league rank.")),
            Some(l) => Ok(l),
        }
    }

    pub async fn current_game_info(&self) -> Result<CurrentGameInfo<'_>> {
        let current_game = self.spectator().await?;

        let mut cgs = Vec::with_capacity(10);

        for p in current_game.participants.iter() {
            let summoner = self
                .api
                .get_summoner(summoner::SummonerEndpointBy::Name(&p.summoner_name))
                .await;

            match summoner {
                Ok(summoner) => {
                    let (champion_id, team_id) = current_game
                        .participants
                        .iter()
                        .find(|p| p.summoner_id == summoner.summoner_info.id)
                        .map(|p| (p.champion_id, p.team_id))
                        .expect("Couldn't map summoner to their champion");

                    cgs.push(SummonerCurrentGameInfo::new(
                        summoner,
                        champion_id,
                        team_id as u8,
                    ));
                }
                Err(e) => error!("{}", e),
            }
        }

        Ok(CurrentGameInfo::new(current_game.game_id as u64, cgs))
    }

    pub async fn match_ids_list(
        &self,
        params: Option<ByPuiidParams>,
    ) -> Result<LeagueMatchList<'_>> {
        let match_list = self
            .api
            .get_match::<Vec<String>>(lol_match::MatchEndpoint::ByPuuid(
                &self.summoner_info.puuid,
                params,
            ))
            .await?;

        Ok(LeagueMatchList::new(match_list, self.api))
    }

    pub async fn champion_win_rate(&self, champion_id: i64) -> Result<ChampionWinRate> {
        let champion_name = self
            .api
            .champion_data
            .champion_list
            .iter()
            .find(|c| c.key == champion_id)
            .map(|c| c.name.to_owned())
            .expect("Couldn't find champion in system.");

        let mut wins = 0;
        let mut losses = 0;

        if let Ok(match_list) = self
            .match_ids_list(Some(ByPuiidParamsBuilder::default().count(25).build()?))
            .await
        {
            for m in match_list.match_info.matches.iter() {
                let match_data = m.match_data().await;

                match match_data {
                    Ok(match_data) => {
                        if let Some(match_result) = match_data
                            .info
                            .participants
                            .iter()
                            .find(|p| {
                                p.puuid == self.summoner_info.puuid && p.champion_id == champion_id
                            })
                            .map(|p| p.win)
                        {
                            if match_result {
                                wins += 1;
                            } else {
                                losses += 1;
                            }
                        }
                    }
                    Err(e) => error!("{}", e),
                }
            }
        }

        Ok(ChampionWinRate::new(
            champion_id,
            champion_name,
            wins,
            losses,
        ))
    }
}
