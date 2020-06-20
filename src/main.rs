#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::env;

use clap::{App, Arg};
use comfy_table::Table;
use tokio::time::Duration;

use crate::api::{Api, ApiRegion};
use crate::endpoints::summoner;
use crate::models::errors::ApiError;
use crate::util::StringExt;

mod api;
mod ddragon;
mod endpoints;
mod models;
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
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Summoner Name")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let api_key = matches.value_of("key").expect("Missing API Key");
    let summoner_name = matches.value_of("name").expect("Missing Summoner name");

    tokio::task::block_in_place(|| do_it(api_key, summoner_name)).await?;
    Ok(())
}

pub async fn do_it(api_key: &str, summoner_name: &str) -> Result<()> {
    let api = Api::new(api_key, ApiRegion::EUW1).await?;

    match api
        .get_summoner(summoner::SummonerEndpoint::ByName(summoner_name))
        .await
    {
        Ok(my_summoner) => {
            let mut games_notified = HashSet::new();

            loop {
                if let Ok((current_game_summoners, game_id)) =
                    my_summoner.current_game_summoners().await
                {
                    let game_notified_id =
                        format!("{}-{}", my_summoner.summoner_info.name, game_id);

                    if !games_notified.contains(&game_notified_id) {
                        let mut all_cwr = Vec::with_capacity(10);

                        for (summoner, cgi) in current_game_summoners.iter() {
                            if let Ok(mut cwr) = summoner.champion_win_rate(cgi.champion_id).await {
                                cwr.set_team_id(cgi.team_id);
                                cwr.set_summoner_name(summoner.summoner_info.name.clone());

                                if let Ok(rank) = summoner.solo_queue_rank().await {
                                    cwr.set_rank(format!(
                                        "{} {}",
                                        rank.tier.to_title_case(),
                                        rank.rank
                                    ));
                                };

                                all_cwr.push(cwr);
                            } else {
                                info!(
                                    "Couldn't find win rate for summoner: {}",
                                    &summoner.summoner_info.name
                                );
                            }
                        }

                        all_cwr.sort_by(|a, b| a.team_id().cmp(&b.team_id()));

                        let player_color = comfy_table::Color::Rgb {
                            r: 239,
                            g: 159,
                            b: 8,
                        };

                        let team_1_colour = comfy_table::Color::Rgb {
                            r: 4,
                            g: 151,
                            b: 211,
                        };

                        let team_2_colour = comfy_table::Color::Rgb {
                            r: 216,
                            g: 58,
                            b: 62,
                        };

                        let no_colour = comfy_table::Color::Reset;

                        let mut table = Table::new();

                        table.set_header(vec![
                            comfy_table::Cell::new("Champion Name")
                                .add_attribute(comfy_table::Attribute::Bold),
                            comfy_table::Cell::new("Win Rate")
                                .add_attribute(comfy_table::Attribute::Bold),
                            comfy_table::Cell::new("Rank (Solo Queue)")
                                .add_attribute(comfy_table::Attribute::Bold),
                        ]);

                        for cwr in all_cwr.iter() {
                            let name_colour =
                                if cwr.summoner_name() == my_summoner.summoner_info.name {
                                    player_color
                                } else {
                                    match cwr.team_id() {
                                        100 => team_1_colour,
                                        200 => team_2_colour,
                                        _ => no_colour,
                                    }
                                };

                            table.add_row(vec![
                                comfy_table::Cell::new(cwr.champion_name()).fg(name_colour),
                                comfy_table::Cell::new(&cwr.win_rate_string()),
                                comfy_table::Cell::new(cwr.rank()),
                            ]);
                        }

                        info!("\n\n{}\n", table);

                        games_notified.insert(game_notified_id);
                    }
                } else {
                    info!("Summoner is not currently in a game.");
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
