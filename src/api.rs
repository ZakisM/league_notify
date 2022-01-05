use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::de::DeserializeOwned;
use strum_macros::{Display, EnumString, EnumVariantNames};

use crate::ddragon::updater::DDragonUpdater;
use crate::endpoints::leagues::LeagueRankEndpoint;
use crate::endpoints::lol_match::MatchEndpoint;
use crate::endpoints::spectator::SpectatorEndpoint;
use crate::endpoints::summoner::SummonerEndpointBy;
use crate::endpoints::Endpoint;
use crate::models::ddragon_champions::ChampionData;
use crate::models::error::MyError;
use crate::models::summoner::{Summoner, SummonerInfo};
use crate::Result;

pub struct Api<'a> {
    key: &'a str,
    client: Client,
    root_endpoint: String,
    v5_root_endpoint: String,
    region: ApiRegion,
    limiters: Vec<Limiter>,
    pub champion_data: ChampionData,
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
        let mut default_headers = HeaderMap::new();

        default_headers.insert(
            "X-Riot-Token",
            HeaderValue::from_str(key).expect("Invalid API Key"),
        );

        let client = ClientBuilder::new()
            .gzip(true)
            .default_headers(default_headers)
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .build()?;

        let l1 = Limiter::new(20, 1);
        let l2 = Limiter::new(100, 120);

        let limiters = vec![l1, l2];

        let ddragon = DDragonUpdater::new().await?;
        let champion_data = ddragon.download_latest_champions().await?;

        Ok(Self {
            key,
            client,
            root_endpoint: region.get_root_endpoint(),
            v5_root_endpoint: region.get_v5_root_endpoint(),
            region,
            limiters,
            champion_data,
        })
    }

    pub async fn get_summoner(&self, endpoint: SummonerEndpointBy<'_>) -> Result<Summoner<'_>> {
        let res = self.call_endpoint(endpoint, false).await?;
        let summoner_info = serde_json::from_str::<SummonerInfo>(&res)?;

        Ok(Summoner::new(summoner_info, self))
    }

    pub async fn get_spectator<T: DeserializeOwned>(
        &self,
        endpoint: SpectatorEndpoint<'_>,
    ) -> Result<T> {
        let res = self.call_endpoint(endpoint, false).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    pub async fn get_match<T: DeserializeOwned>(&self, endpoint: MatchEndpoint<'_>) -> Result<T> {
        let res = self.call_endpoint(endpoint, true).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    pub async fn get_leagues<T: DeserializeOwned>(
        &self,
        endpoint: LeagueRankEndpoint<'_>,
    ) -> Result<T> {
        let res = self.call_endpoint(endpoint, false).await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }

    async fn call_endpoint(&self, endpoint: impl Endpoint, is_v5: bool) -> Result<String> {
        let endpoint_url = endpoint.url();

        let mut attempts = 0;

        'outer: loop {
            if attempts == 3 {
                return Err(MyError::Other(anyhow!(
                    "Failed to make request: {}",
                    endpoint_url
                )));
            }

            for l in self.limiters.iter() {
                // Wait until limiters no longer give us an error.
                while l.take().is_err() {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }

            let root_endpoint = if !is_v5 {
                &self.root_endpoint
            } else {
                &self.v5_root_endpoint
            };

            let res = self
                .client
                .get(format!("{}/{}", root_endpoint, endpoint_url))
                .send()
                .await?;

            attempts += 1;

            match res.status() {
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Some(retry_after) = res.headers().get("retry-after") {
                        let delay = retry_after.to_str().unwrap().parse::<u64>().unwrap();

                        // Empty all buckets to stop any requests in other threads from continuing
                        for l in self.limiters.iter() {
                            l.empty();
                        }

                        info!(
                            "TOO_MANY_REQUESTS received - Delaying for {} seconds.",
                            delay
                        );

                        for l in self.limiters.iter() {
                            l.refill();
                        }

                        tokio::time::sleep(Duration::from_secs(delay)).await;
                    }
                }
                StatusCode::NOT_FOUND => {
                    return Err(MyError::Other(anyhow!(
                        "No data was found for endpoint: {}",
                        endpoint_url
                    )));
                }
                StatusCode::OK => {
                    return Ok(res.text().await?);
                }
                _ => {
                    //try again in 1 sec
                    tokio::time::sleep(Duration::from_millis(500)).await;
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

    fn get_v5_root_endpoint(&self) -> String {
        let routing_value = match self {
            ApiRegion::NA1 | ApiRegion::BR1 | ApiRegion::LA1 | ApiRegion::LA2 | ApiRegion::OC1 => {
                "americas"
            }
            ApiRegion::EUN1 | ApiRegion::EUW1 | ApiRegion::RU | ApiRegion::TR1 => "europe",
            ApiRegion::JP1 | ApiRegion::KR => "asia",
        };

        format!("https://{}.api.riotgames.com", routing_value)
    }
}

#[derive(Debug)]
pub struct Limiter {
    initial_size: usize,
    bucket: Arc<AtomicUsize>,
}

impl Limiter {
    pub fn new(size: usize, refill_time: u64) -> Self {
        let bucket = AtomicUsize::new(size);

        let bucket_ptr = Arc::new(bucket);

        let refill_ptr = Arc::clone(&bucket_ptr);

        tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(refill_time)).await;

                refill_ptr.store(size, Ordering::Release)
            }
        });

        Limiter {
            initial_size: size,
            bucket: Arc::clone(&bucket_ptr),
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
            Err(MyError::Other(anyhow!("Bucket empty")))
        }
    }
}
