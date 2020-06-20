use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::endpoints::ddragon;
use crate::models::ddragon_champions::ChampionData;
use crate::Result;

#[derive(Debug)]
pub struct DDragonUpdater {
    client: Client,
    version: String,
}

impl DDragonUpdater {
    pub async fn new() -> Result<Self> {
        let client = Client::new();

        let res = client
            .get(&ddragon::DDragonEndpoint::Version.url())
            .send()
            .await?
            .text()
            .await?;

        let version = serde_json::from_str::<Vec<String>>(&res)?
            .get(0)
            .expect("Missing version data from DDragon.")
            .to_string();

        Ok(DDragonUpdater { client, version })
    }

    pub async fn download_latest_champions(&self) -> Result<ChampionData> {
        let data = self
            .call_endpoint(&ddragon::DDragonEndpoint::ChampionData(&self.version))
            .await?;

        Ok(data)
    }

    async fn call_endpoint<T: DeserializeOwned>(
        &self,
        endpoint: &ddragon::DDragonEndpoint<'_>,
    ) -> Result<T> {
        let res = self
            .client
            .get(&endpoint.url())
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<T>(&res)?)
    }
}
