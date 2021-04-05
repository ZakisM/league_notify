use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
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

            for l in self.limiters.iter() {
                // Wait until limiters no longer give us an error.
                while l.take().is_err() {
                    tokio::time::delay_for(Duration::from_millis(100)).await;
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

                        // Empty all buckets to stop any requests in other threads from continuing
                        for l in self.limiters.iter() {
                            l.empty();
                        }

                        debug!(
                            "TOO_MANY_REQUESTS received - Delaying for {} seconds.",
                            delay
                        );

                        for l in self.limiters.iter() {
                            l.refill();
                        }

                        tokio::time::delay_for(Duration::from_secs(delay)).await;
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
#[allow(dead_code, clippy::upper_case_acronyms)]
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
    initial_size: usize,
    bucket: Arc<AtomicUsize>,
    refill_time: u64,
}

impl Limiter {
    pub fn new(size: usize, refill_time: u64) -> Self {
        let bucket = AtomicUsize::new(size);

        let bucket_ptr = Arc::new(bucket);

        let refill_ptr = Arc::clone(&bucket_ptr);

        tokio::task::spawn(async move {
            loop {
                tokio::time::delay_for(Duration::from_secs(refill_time)).await;

                refill_ptr.store(size, Ordering::Release)
            }
        });

        Limiter {
            initial_size: size,
            bucket: Arc::clone(&bucket_ptr),
            refill_time,
        }
    }

    pub fn refill(&self) {
        self.bucket.store(self.initial_size, Ordering::Release);
    }

    pub fn empty(&self) {
        self.bucket.store(0, Ordering::Release);
    }

    pub fn take(&self) -> Result<()> {
        let current = self.bucket.load(Ordering::Acquire);

        if current > 0 {
            self.bucket.fetch_sub(1, Ordering::Release);
            Ok(())
        } else {
            Err(ApiError::new("Bucket empty"))
        }
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
                for l in lmtrs.iter() {
                    // Wait until limiters no longer give us an error.
                    while l.take().is_err() {
                        tokio::time::delay_for(Duration::from_millis(100)).await;
                    }
                }
            });
        }

        while let Some(_) = all_threads.next().await {}
    }
}
