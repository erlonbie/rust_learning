use clap::{App, Arg};
use console::style;
use serde::Deserialize;
use std::{format, println};

#[derive(Deserialize, Debug)]
struct Pokemon {
    id: u64,
    name: String,
    types: Vec<TypeSlot>,
    height: u64,
    weight: u64,
}

#[derive(Deserialize, Debug)]
struct TypeSlot {
    #[serde(rename = "type")]
    type_object: Type,
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

    get_pokemon(pokemon);
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
    println!("Height: {}", style(pokemon.height).bold().magenta());
    println!("Weight: {}", style(pokemon.weight).bold().cyan());
    print!("Types: [");
    for type_slot in pokemon.types {
        print!(" {}, ", style(type_slot.type_object.name).bold().green());
    }
    println!("]");
}

fn request_api(pokemon_name: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
    let response = reqwest::blocking::get(url)?.json::<Pokemon>()?;
    Ok(response)
}

