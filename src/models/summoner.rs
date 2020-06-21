use core::fmt;

use getset::Getters;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;

use crate::api::Api;
use crate::endpoints::lol_match::{ByAccountIdParams, ByAccountIdParamsBuilder};
use crate::endpoints::{leagues, lol_match, spectator, summoner};
use crate::models::champion::ChampionWinRate;
use crate::models::errors::ApiError;
use crate::models::leagues::LeagueRank;
use crate::models::lol_match::{LeagueMatchList, MatchInfo};
use crate::models::spectator::SpectatorInfo;
use crate::Result;

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i64,
    revision_date: i64,
    summoner_level: i64,
}

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct SummonerCurrentGameInfo<'a> {
    summoner: Summoner<'a>,
    champion_id: u32,
    team_id: u8,
}

impl<'a> SummonerCurrentGameInfo<'a> {
    pub fn new(summoner: Summoner<'a>, champion_id: u32, team_id: u8) -> Self {
        SummonerCurrentGameInfo {
            summoner,
            champion_id,
            team_id,
        }
    }
}

#[derive(Getters)]
#[get = "pub"]
pub struct CurrentGameInfo<'a> {
    game_id: u64,
    summoners: Vec<SummonerCurrentGameInfo<'a>>,
}

impl<'a> CurrentGameInfo<'a> {
    pub fn new(game_id: u64, summoners: Vec<SummonerCurrentGameInfo<'a>>) -> Self {
        CurrentGameInfo { game_id, summoners }
    }
}

#[derive(Getters)]
#[get = "pub"]
pub struct Summoner<'a> {
    summoner_info: SummonerInfo,
    #[get]
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
            .find(|l| l.queue_type() == "RANKED_SOLO_5x5")
        {
            None => Err(ApiError::new("Could not find league rank.")),
            Some(l) => Ok(l),
        }
    }

    pub async fn current_game_info(&self) -> Result<CurrentGameInfo<'_>> {
        let current_game = self.spectator().await?;

        let mut all_summoners = futures::stream::FuturesUnordered::new();

        current_game.participants().iter().for_each(|p| {
            all_summoners.push(
                self.api
                    .get_summoner(summoner::SummonerEndpoint::ByName(&p.summoner_name())),
            )
        });

        let mut cgs = Vec::with_capacity(10);

        while let Some(summoner) = all_summoners.next().await {
            match summoner {
                Ok(summoner) => {
                    let (champion_id, team_id) = current_game
                        .participants()
                        .iter()
                        .find(|p| *p.summoner_id() == summoner.summoner_info.id)
                        .map(|p| (*p.champion_id() as u32, p.team_id()))
                        .expect("Couldn't map summoner to their champion");

                    cgs.push(SummonerCurrentGameInfo::new(
                        summoner,
                        champion_id,
                        *team_id as u8,
                    ));
                }
                Err(e) => error!("{}", e),
            }
        }

        Ok(CurrentGameInfo::new(*current_game.game_id() as u64, cgs))
    }

    pub async fn match_list(
        &self,
        params: Option<ByAccountIdParams>,
    ) -> Result<LeagueMatchList<'_>> {
        let match_list = self
            .api
            .get_match::<MatchInfo>(lol_match::MatchEndpoint::ByAccountId(
                &self.summoner_info.account_id,
                params,
            ))
            .await?;

        Ok(LeagueMatchList::new(match_list, self.api))
    }

    pub async fn champion_win_rate(&self, champion_id: u32) -> Result<ChampionWinRate> {
        let champion_name = self
            .api
            .champion_data()
            .champion_list()
            .iter()
            .find(|c| *c.key() == champion_id)
            .map(|c| c.name().to_owned())
            .expect("Couldn't find champion in system.");

        let mut wins = 0;
        let mut losses = 0;

        if let Ok(match_list) = self
            .match_list(Some(
                ByAccountIdParamsBuilder::default()
                    .champion(champion_id)
                    .end_index(10)
                    .build()?,
            ))
            .await
        {
            let mut all_matches = futures::stream::FuturesUnordered::new();

            match_list.match_info().matches().iter().for_each(|m| {
                all_matches.push(m.match_data());
            });

            while let Some(m) = all_matches.next().await {
                match m {
                    Ok(m) => {
                        let participant_id = m
                            .participant_identities()
                            .iter()
                            .find(|p| *p.player().summoner_id() == self.summoner_info.id)
                            .map(|p| p.participant_id())
                            .expect("Could not find player info");

                        let match_result = m
                            .participants()
                            .iter()
                            .find(|p| p.participant_id() == participant_id)
                            .map(|p| p.stats().win())
                            .expect("Couldn't find match result");

                        if *match_result {
                            wins += 1;
                        } else {
                            losses += 1;
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
