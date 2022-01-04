use comfy_table::Table;

use crate::models::champion::ChampionWinRate;
use crate::models::summoner::Summoner;

pub fn generate_table(my_summoner: &Summoner<'_>, results: Vec<ChampionWinRate>) -> Table {
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
        comfy_table::Cell::new("Champion Name").add_attribute(comfy_table::Attribute::Bold),
        comfy_table::Cell::new("Win Rate").add_attribute(comfy_table::Attribute::Bold),
        comfy_table::Cell::new("Rank (Solo Queue)").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for cwr in results.iter() {
        let name_colour = if cwr.summoner_name == my_summoner.summoner_info.name {
            player_color
        } else {
            match cwr.team_id {
                100 => team_1_colour,
                200 => team_2_colour,
                _ => no_colour,
            }
        };

        table.add_row(vec![
            comfy_table::Cell::new(cwr.champion_name.clone()).fg(name_colour),
            comfy_table::Cell::new(&cwr.win_rate_string()),
            comfy_table::Cell::new(cwr.rank.clone()),
        ]);
    }

    table
}
