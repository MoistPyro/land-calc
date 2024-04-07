use bulk_files::get_bulk_from_scryfall;
use card::{CardObject, SearchResult};
use search::get_from_scryfall;

use slint::SharedString;
use std::{fs::read_to_string, io};

mod bulk_files;
mod card;
mod search;

slint::slint! {
    import { CheckBox , Button, GroupBox, LineEdit} from "std-widgets.slint";

    export component AppWindow inherits Window {
        out property <bool> commander;
        out property <bool> companion;
        out property <string> cards;
        out property <string> ramp;
        out property <string> draw;
        in property <string> answer;
        in property <string> info;
        in property <string> errors;
        callback do_the_thing();
        VerticalLayout {
            spacing: 5px;
            padding: 5px;

            Text {
                font-size: 14px;
                color: red;
                horizontal-alignment: center;
                text: errors;
            }

            Text {
                font-size: 14px;
                horizontal-alignment: center;
                text: info;
            }

            GroupBox {
                title: "Outside the Game";

                HorizontalLayout {
                    spacing: 5px;
                    CheckBox {
                        height: 25px;
                        text: "commander";
                        toggled => {
                            root.commander = !root.commander;
                        }
                    }
                    CheckBox {
                        height: 25px;
                        text: "companion";
                        toggled => {
                            root.companion = !root.companion;
                        }
                    }
                }
            }

            LineEdit {
                font-size: 12px;
                input-type: number;
                placeholder-text: "deck size";
                text: "";
                edited => {
                    root.cards = self.text;
                }
            }

            LineEdit {
                font-size: 12px;
                input-type: number;
                placeholder-text: "ramp + dorks";
                text: "";
                edited => {
                    root.ramp = self.text;
                }
            }

            LineEdit {
                font-size: 12px;
                input-type: number;
                placeholder-text: "cheap draw";
                text: "";
                edited => {
                    root.draw = self.text;
                }
            }

            Text {
                font-size: 14px;
                horizontal-alignment: center;
                text: answer;
            }

            Button {
                text: "submit";
                clicked => {
                    root.do_the_thing();
                }
            }
        }
    }
}

const FILE: &str = "list.txt";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let list = read_decklist().expect("no file called 'list.txt' found");

    // let _temp = get_bulk_from_scryfall().await?;
    // return Ok(());

    let mut warnings = vec!["warnings:".to_string()];

    let search_results: Vec<(u32, SearchResult)> = get_from_scryfall(list).await?;

    let cards: Vec<(u32, CardObject)> = search_results
        .iter()
        .filter_map(|(amount, search_result)| match search_result {
            SearchResult::MultipleHits(_, _, c) => Some((*amount, c.clone())),
            SearchResult::OneHit(c) => Some((*amount, c.clone())),
            SearchResult::NoHits(_) => None,
        })
        .collect();

    let mut errors: Vec<String> = search_results
        .iter()
        .filter_map(|(_, search_result)| match search_result {
            SearchResult::MultipleHits(q, i, card) => Some(format!(
                "{} hits for {}, using the first one: {}",
                i, q, card.name
            )),
            SearchResult::OneHit(_) => None,
            SearchResult::NoHits(q) => Some(format!("No card with name {} found", q)),
        })
        .collect();

    warnings.append(&mut errors);
    let warning_display: String = warnings.join("\n");

    let number_of_spells: usize = cards.iter().filter(|(_, c)| c.is_nonland()).count();
    let lands_func = recommended_lands_static_cards(cards);

    run_app(lands_func, number_of_spells, warning_display).expect("slint did not initialize");
    Ok(())
}

fn recommended_lands(
    total_cards: u32,
    list: &Vec<(u32, CardObject)>,
    ramp: u32,
    draw: u32,
    cmdr_cmp: u32,
) -> f64 {
    let total_mana_value: u32 = list
        .iter()
        .map(|(a, c)| *a * c.cmc.as_f64().unwrap() as u32)
        .fold(0, |a, b| a + b);
    let average_mv = total_mana_value as f64 / total_cards as f64;

    (total_cards as f64 / 60.0) * (19.59 + 1.9 * average_mv + 0.27 * cmdr_cmp as f64)
        - 0.28 * (ramp + draw) as f64
}

fn recommended_lands_static_cards<'a>(
    list: Vec<(u32, CardObject)>,
) -> impl Fn(u32, u32, u32, u32) -> f64 + 'a {
    move |total_cards: u32, ramp: u32, draw: u32, cmdr_cmp: u32| {
        recommended_lands(total_cards, &list, ramp, draw, cmdr_cmp)
    }
}

fn parse_shared_string_u32(s: SharedString) -> u32 {
    if s.is_empty() {
        return 0;
    } else {
        return s.parse().unwrap();
    }
}

fn run_app<F>(
    land_finder_function: F,
    spells: usize,
    errors: String,
) -> Result<(), slint::PlatformError>
where
    F: Fn(u32, u32, u32, u32) -> f64 + 'static,
{
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak().unwrap();

    ui_handle.set_info(
        format!(
            "Detected a deck with {} spells.\nFill in the following info\nto get a recommandation.",
            spells
        )
        .into(),
    );
    ui_handle.set_errors(errors.into());
    ui_handle.set_answer("tries to read 'list.txt'".into());

    ui.on_do_the_thing(move || {
        let commander = ui_handle.get_commander();
        let companion = ui_handle.get_companion();

        let total_cards = parse_shared_string_u32(ui_handle.get_cards());
        let ramp = parse_shared_string_u32(ui_handle.get_ramp());
        let draw = parse_shared_string_u32(ui_handle.get_draw());
        let cmdr_cmp = if commander && companion { 2 } else { 0 };

        let recommended_lands = land_finder_function(total_cards, ramp, draw, cmdr_cmp);

        let answer_str: String = format!("play {} lands", recommended_lands);
        ui_handle.set_answer(answer_str.into());
    });

    ui.run()?;

    Ok(())
}

fn read_decklist() -> io::Result<Vec<(u32, String)>> {
    let file = read_to_string(FILE)?;
    let lines: Vec<String> = file
        .split(|c| c == '\n' || c == '\r')
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();

    let mut list: Vec<(u32, String)> = Vec::new();

    for line in lines {
        let num: u32 = line.split(" ").next().unwrap().parse().unwrap(); //the first symbol of each line is the amount of this card in the list
        let name_parts: Vec<&str> = line
            .split(" ")
            .skip(1) //skip amount of this card
            .filter(|s| s.len() > 0) //remove empty strs made by double whitespace
            .take_while(|s| !s.contains("(")) //only read until the set code (AAA)
            .collect();
        let card: String = name_parts.join(" ");
        let regex: String = format!(r"name:/^{}$/", card);
        list.push((num, regex));
    }

    Ok(list)
}
