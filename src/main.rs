#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::env;

use clap::{App, Arg};
use strum::VariantNames;
use tokio::time::Duration;

use crate::api::{Api, ApiRegion};
use crate::endpoints::summoner;
use crate::models::errors::ApiError;
use crate::util::StringExt;

mod api;
mod ddragon;
mod endpoints;
mod models;
mod table;
mod util;

type Result<T> = std::result::Result<T, ApiError>;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "INFO");

    pretty_env_logger::init_timed();

    let matches = App::new("League Notifier")
        .version("1.0")
        .author("Zak")
        .about("Gives you info about current players in your game.")
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Riot API Key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("region")
                .short("r")
                .long("region")
                .help("API Region")
                .takes_value(true)
                .possible_values(&ApiRegion::VARIANTS)
                .required(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Summoner Name")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let api_key = matches.value_of("key").expect("Missing API Key");

    let region = matches
        .value_of("region")
        .expect("Missing API Region")
        .parse()
        .map_err(|_| ApiError::new("Invalid API Region"))?;

    let summoner_name = matches.value_of("name").expect("Missing Summoner name");

    tokio::task::block_in_place(|| track_summoner(api_key, region, summoner_name)).await?;
    Ok(())
}

pub async fn track_summoner(api_key: &str, region: ApiRegion, summoner_name: &str) -> Result<()> {
    let api = Api::new(api_key, region).await?;

    match api
        .get_summoner(summoner::SummonerEndpoint::ByName(summoner_name))
        .await
    {
        Ok(my_summoner) => {
            let mut games_notified = HashSet::new();

            loop {
                if let Ok(cgi) = my_summoner.current_game_info().await {
                    let game_notified_id =
                        format!("{}-{}", &my_summoner.summoner_info().name(), cgi.game_id());

                    if !games_notified.contains(&game_notified_id) {
                        info!("Game detected, loading info...");

                        let mut results = Vec::with_capacity(10);

                        for summoner_current_game_info in cgi.summoners().iter() {
                            let summoner = summoner_current_game_info.summoner();

                            if let Ok(mut cwr) = summoner
                                .champion_win_rate(*summoner_current_game_info.champion_id())
                                .await
                            {
                                cwr.set_team_id(*summoner_current_game_info.team_id());
                                cwr.set_summoner_name(summoner.summoner_info().name().clone());

                                if let Ok(rank) = summoner.solo_queue_rank().await {
                                    cwr.set_rank(format!(
                                        "{} {}",
                                        rank.tier().to_owned().to_title_case(),
                                        rank.rank()
                                    ));
                                };

                                results.push(cwr);
                            } else {
                                info!(
                                    "Couldn't find win rate for summoner: {}",
                                    &summoner.summoner_info().name()
                                );
                            }
                        }

                        results.sort();

                        let table = table::generate_table(&my_summoner, results);

                        info!("\n\n{}\n", table);

                        games_notified.insert(game_notified_id);
                    }
                }

                tokio::time::delay_for(Duration::from_secs(30)).await;
            }
        }
        Err(e) => {
            error!("Could not get summoner: {}", e);
        }
    };

    Ok(())
}
