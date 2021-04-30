use std::collections::HashMap;

use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
pub struct ChampionData {
    #[serde(rename = "type")]
    data_type: String,
    format: String,
    version: String,
    #[serde(deserialize_with = "champion_list_deserializer")]
    #[serde(rename = "data")]
    champion_list: Vec<Champion>,
}

fn champion_list_deserializer<'de, D>(deserializer: D) -> Result<Vec<Champion>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let data: HashMap<String, Champion> = HashMap::deserialize(deserializer)?;

    Ok(data.into_iter().map(|(_, v)| v).collect())
}

fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse()
        .expect("Failed to deserialize champion key to u64."))
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    version: String,
    id: String,
    #[serde(deserialize_with = "string_to_u64")]
    key: u64,
    name: String,
    title: String,
    blurb: String,
    info: Info,
    image: Image,
    tags: Vec<String>,
    partype: String,
    stats: Stats,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Info {
    attack: i64,
    defense: i64,
    magic: i64,
    difficulty: i64,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Image {
    full: String,
    sprite: String,
    group: String,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
#[get = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    hp: f64,
    hpperlevel: i64,
    mp: f64,
    mpperlevel: f64,
    movespeed: i64,
    armor: f64,
    armorperlevel: f64,
    spellblock: f64,
    spellblockperlevel: f64,
    attackrange: i64,
    hpregen: f64,
    hpregenperlevel: f64,
    mpregen: f64,
    mpregenperlevel: f64,
    crit: i64,
    critperlevel: i64,
    attackdamage: f64,
    attackdamageperlevel: f64,
    attackspeedperlevel: f64,
    attackspeed: f64,
}
