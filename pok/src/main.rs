use clap::{App, Arg};

mod api;
mod color;
mod models;
mod print;
mod utils;

use crate::api::get_pokemon;

fn main() {
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

    let pokemon = app
        .value_of("pokemon_name")
        .expect("Could not get the pokemon name");

    get_pokemon(pokemon.to_lowercase().as_str());
}
