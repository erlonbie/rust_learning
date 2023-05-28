use clap::{App, Arg};
use console::style;
use serde::Deserialize;
use std::{format, println};

#[derive(Deserialize, Debug)]
struct Pokemon {
    id: u64,
    name: String,
    types: Vec<TypeSlot>,
    stats: Vec<StatSlot>,
    height: f32,
    weight: f32,
}

#[derive(Deserialize, Debug)]
struct TypeSlot {
    #[allow(dead_code)]
    slot: u64,
    #[serde(rename = "type")]
    type_object: Type,
}

#[derive(Deserialize, Debug)]
struct StatSlot {
    base_stat: u64,
    #[allow(dead_code)]
    effort: u64,
    #[serde(rename = "stat")]
    stat_object: Stat,
}

#[derive(Deserialize, Debug)]
struct Stat {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Type {
    name: String,
}

fn main() {
    println!("{}", style("Hello, world!").yellow());

    let app = App::new("Dex")
        .version("0.1.0")
        .author("Erlon P. Bié <erlon@gmail.com>")
        .about("Pokedex CLI")
        .arg(
            Arg::with_name("pokemon_name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of pokemon"),
        )
        .get_matches();

    let pokemon = app.value_of("pokemon_name").unwrap();

    get_pokemon(pokemon.to_lowercase().as_str());
}

fn get_pokemon(pokemon_name: &str) {
    match request_api(pokemon_name) {
        Ok(pokemon) => handle_pokemon(pokemon),
        Err(e) => println!("Pokemon not found: {}", e),
    }
}

fn handle_pokemon(pokemon: Pokemon) {
    println!("Pokemon: {}", style(pokemon.name).bold().yellow());
    println!("Id: {}", style(pokemon.id).bold().red());
    println!("Height: {} m", style(pokemon.height/10.0).bold().magenta());
    println!("Weight: {} kg", style(pokemon.weight/10.0).bold().cyan());
    print!("Types: [");
    for type_slot in pokemon.types {
        print!(" {}, ", style(type_slot.type_object.name).bold().green());
    }
    println!("]");
    print!("Stats:");
    println!();
    for stat_slot in pokemon.stats {
        println!("\t {}: {}, ", style(stat_slot.stat_object.name).bold().blue(), style(stat_slot.base_stat).bold().red());
    }
}

fn request_api(pokemon_name: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
    let response = reqwest::blocking::get(url)?.json::<Pokemon>()?;
    Ok(response)
}

