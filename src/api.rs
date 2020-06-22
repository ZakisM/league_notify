use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

use getset::Getters;
use reqwest::header::HeaderValue;
use reqwest::{Client, Method, Request, StatusCode};
use serde::de::DeserializeOwned;
use strum_macros::{Display, EnumString, EnumVariantNames};

use crate::ddragon::updater::DDragonUpdater;
use crate::endpoints::leagues::LeagueRankEndpoint;
use crate::endpoints::lol_match::MatchEndpoint;
use crate::endpoints::spectator::SpectatorEndpoint;
use crate::endpoints::summoner::SummonerEndpoint;
use crate::models::ddragon_champions::ChampionData;
use crate::models::errors::ApiError;
use crate::models::summoner::{Summoner, SummonerInfo};
use crate::Result;

#[derive(Getters)]
#[get = "pub"]
pub struct Api<'a> {
    key: &'a str,
    client: Client,
    root_endpoint: String,
    region: ApiRegion,
    limiters: Vec<Limiter>,
    champion_data: ChampionData,
}

impl fmt::Debug for Api<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Key: {} - Region: {}", self.key, self.region)
    }
}

impl fmt::Display for Api<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Key: {} - Region: {}", self.key, self.region)
    }
}

impl<'a> Api<'a> {
    pub async fn new(key: &'a str, region: ApiRegion) -> Result<Api<'a>> {
        let l1 = Limiter::new(20, 1);
        let l2 = Limiter::new(100, 120);

        let limiters = vec![l1, l2];

        let ddragon = DDragonUpdater::new().await?;
        let champion_data = ddragon.download_latest_champions().await?;

        Ok(Self {
            key,
            client: Client::new(),
            root_endpoint: region.get_root_endpoint(),
            region,
            limiters,
            champion_data,
        })
    }

    pub async fn get_summoner(&self, endpoint: SummonerEndpoint<'_>) -> Result<Summoner<'_>> {
        let res = self.call_endpoint(endpoint.url()).await?;
        let summoner_info = serde_json::from_str::<SummonerInfo>(&res)?;

        Ok(Summoner::new(summoner_info, self))
    }

    pub async fn get_spectator<T: DeserializeOwned>(
        &self,
        endpoint: SpectatorEndpoint<'_>,
    ) -> Result<T> {
        let res = self.call_endpoint(endpoint.url()).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    pub async fn get_match<T: DeserializeOwned>(&self, endpoint: MatchEndpoint<'_>) -> Result<T> {
        let res = self.call_endpoint(endpoint.url()).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    pub async fn get_leagues<T: DeserializeOwned>(
        &self,
        endpoint: LeagueRankEndpoint<'_>,
    ) -> Result<T> {
        let res = self.call_endpoint(endpoint.url()).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    async fn call_endpoint(&self, endpoint_url: String) -> Result<String> {
        let mut attempts = 0;

        'outer: loop {
            if attempts == 3 {
                return Err(ApiError::new(format!(
                    "Failed to make request: {}",
                    endpoint_url
                )));
            }

            let mut is_ok = true;

            for l in self.limiters.iter() {
                if l.take().is_err() {
                    //delay task for minimum refill time + 1s ish
                    tokio::time::delay_for(Duration::from_millis(250)).await;
                    is_ok = false;
                }

                if !is_ok {
                    continue 'outer;
                }
            }

            let mut req = Request::new(
                Method::GET,
                format!("{}/{}", self.root_endpoint, endpoint_url)
                    .parse()
                    .expect("Invalid URL"),
            );

            req.headers_mut().insert(
                "X-Riot-Token",
                HeaderValue::from_str(self.key).expect("Invalid API Key"),
            );

            let res = self.client.execute(req).await?;

            attempts += 1;

            match res.status() {
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Some(retry_after) = res.headers().get("retry-after") {
                        let delay = retry_after.to_str().unwrap().parse::<u64>().unwrap();

                        for l in self.limiters.iter() {
                            l.drain_all()?;
                        }

                        tokio::time::delay_for(Duration::from_secs(delay)).await;

                        for l in self.limiters.iter() {
                            l.refill_all()?;
                        }
                    }
                }
                StatusCode::NOT_FOUND => {
                    return Err(ApiError::new(format!(
                        "No data was found for endpoint: {}",
                        endpoint_url
                    )));
                }
                StatusCode::OK => {
                    return Ok(res.text().await?);
                }
                _ => {
                    //try again in 1 sec
                    tokio::time::delay_for(Duration::from_millis(500)).await;
                    continue 'outer;
                }
            }
        }
    }
}

#[derive(Debug, Display, EnumString, EnumVariantNames)]
#[allow(dead_code)]
pub enum ApiRegion {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    NA1,
    OC1,
    RU,
    TR1,
}

impl ApiRegion {
    fn get_root_endpoint(&self) -> String {
        format!(
            "https://{}.api.riotgames.com",
            self.to_string().to_lowercase()
        )
    }
}

#[derive(Debug)]
pub struct Limiter {
    channel: (
        crossbeam::channel::Sender<()>,
        crossbeam::channel::Receiver<()>,
    ),
    refill_time: u64,
}

impl Limiter {
    pub fn new(size: usize, refill_time: u64) -> Self {
        let channel = crossbeam::bounded(size);

        let tx_clone = channel.0.clone();

        tokio::task::spawn(async move {
            loop {
                tokio::time::delay_for(Duration::from_secs(refill_time)).await;

                while !tx_clone.is_full() {
                    if tx_clone.try_send(()).is_err() {
                        error!("Error trying to refill");
                    }
                }
            }
        });

        for _ in 0..size {
            channel.0.send(()).expect("Failed to create limiter.");
        }

        Limiter {
            channel,
            refill_time,
        }
    }

    pub fn give(&self) -> Result<()> {
        if self.channel.0.try_send(()).is_err() {
            Err(ApiError::new("Error trying to refill."))
        } else {
            Ok(())
        }
    }

    pub fn take(&self) -> Result<()> {
        if self.channel.1.try_recv().is_err() {
            Err(ApiError::new("Have consumed all available items."))
        } else {
            Ok(())
        }
    }

    pub fn drain_all(&self) -> Result<()> {
        while !self.channel.0.is_empty() {
            self.take()?
        }

        Ok(())
    }

    pub fn refill_all(&self) -> Result<()> {
        while !self.channel.0.is_full() {
            self.give()?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::stream::StreamExt;
    use tokio::time::Duration;

    use crate::api::Limiter;

    #[tokio::test]
    async fn test_limiter() {
        let lmtr1 = Limiter::new(20, 1);
        let lmtr2 = Limiter::new(100, 5);

        let lmtrs = vec![lmtr1, lmtr2];

        let mut all_threads = futures::stream::FuturesUnordered::new();

        for _ in 0..102 {
            all_threads.push(async {
                'outer: loop {
                    let mut is_ok = true;

                    for l in lmtrs.iter() {
                        if let Err(e) = l.take() {
                            tokio::time::delay_for(Duration::from_secs(1)).await;
                            is_ok = false;
                        }

                        if !is_ok {
                            continue 'outer;
                        }
                    }

                    if is_ok {
                        break;
                    }
                }
            });
        }

        while let Some(r) = all_threads.next().await {}
    }

    #[tokio::test]
    async fn test_drain_all() {
        let lmtr = Limiter::new(50, 1);

        lmtr.drain_all().expect("Failed to drain all");

        assert!(lmtr.channel.0.is_empty());
    }

    #[tokio::test]
    async fn test_refill_all() {
        let lmtr = Limiter::new(50, 1);

        lmtr.drain_all().expect("Failed to drain all");

        assert!(lmtr.channel.0.is_empty());

        lmtr.refill_all().expect("Failed to refill all");

        assert!(lmtr.channel.0.is_full());
    }
}
