use clap::{App, Arg};
use console::style;
use serde::Deserialize;
use std::{fmt, format, println};
use termion::{color, style};
use std::collections::HashMap;

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

    let pokemon = app.value_of("pokemon_name").expect("Could not get the pokemon name");

    get_pokemon(pokemon.to_lowercase().as_str());
}



fn get_pokemon(pokemon_name: &str) {
    match request_pokemon(pokemon_name) {
        Ok(pokemon) => handle_pokemon(pokemon),
        Err(_) => match request_pokemon_species(pokemon_name) {
            // BUG: this is not working yet <31-05-23 11:46, erlonbie> //
            Ok(pokemon) => handle_pokemon(pokemon),
            Err(e) => println!("Pokemon not found: {}", e),
        }
    }
}

enum PokemonTypeColor {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl fmt::Display for PokemonTypeColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PokemonTypeColor::Normal => {
                write!( f, "{}Normal{}", color::Fg(color::Rgb(168, 144, 240)), style::Reset )
            },
            PokemonTypeColor::Fire => {
                write!( f, "{}Fire{}", color::Fg(color::Rgb(240, 128, 48)), style::Reset )
            },
            PokemonTypeColor::Water => {
                write!( f, "{}Water{}", color::Fg(color::Rgb(104, 144, 240)), style::Reset )
            },
            PokemonTypeColor::Electric => {
                write!( f, "{}Electric{}", color::Fg(color::Rgb(248, 208, 48)), style::Reset )
            },
            PokemonTypeColor::Grass => {
                write!( f, "{}Grass{}", color::Fg(color::Rgb(120, 200, 80)), style::Reset )
            },
            PokemonTypeColor::Ice => {
                write!( f, "{}Ice{}", color::Fg(color::Rgb(152, 216, 216)), style::Reset )
            },
            PokemonTypeColor::Fighting => {
                write!( f, "{}Fighting{}", color::Fg(color::Rgb(192, 48, 40)), style::Reset )
            },
            PokemonTypeColor::Poison => {
                write!( f, "{}Poison{}", color::Fg(color::Rgb(160, 64, 160)), style::Reset )
            },
            PokemonTypeColor::Ground => {
                write!( f, "{}Ground{}", color::Fg(color::Rgb(224, 192, 104)), style::Reset )
            },
            PokemonTypeColor::Flying => {
                write!( f, "{}Flying{}", color::Fg(color::Rgb(168, 144, 240)), style::Reset )
            },
            PokemonTypeColor::Psychic => {
                write!( f, "{}Psychic{}", color::Fg(color::Rgb(248, 88, 136)), style::Reset )
            },
            PokemonTypeColor::Bug => {
                write!( f, "{}Bug{}", color::Fg(color::Rgb(168, 184, 32)), style::Reset )
            },
            PokemonTypeColor::Rock => {
                write!( f, "{}Rock{}", color::Fg(color::Rgb(184, 160, 56)), style::Reset )
            },
            PokemonTypeColor::Ghost => {
                write!( f, "{}Ghost{}", color::Fg(color::Rgb(112, 88, 152)), style::Reset )
            },
            PokemonTypeColor::Dragon => {
                write!( f, "{}Dragon{}", color::Fg(color::Rgb(112, 56, 248)), style::Reset )
            },
            PokemonTypeColor::Dark => {
                write!( f, "{}Dark{}", color::Fg(color::Rgb(112, 88, 72)), style::Reset )
            },
            PokemonTypeColor::Steel => {
                write!( f, "{}Steel{}", color::Fg(color::Rgb(184, 184, 208)), style::Reset )
            },
            PokemonTypeColor::Fairy => {
                write!( f, "{}Fairy{}", color::Fg(color::Rgb(238, 153, 172)), style::Reset )
            },
        }
    }
}

fn handle_type_color(pokemon: &str) {
    match pokemon {
        "normal" => print!(" {}", PokemonTypeColor::Normal),
        "fire" => print!(" {}", PokemonTypeColor::Fire),
        "water" => print!(" {}", PokemonTypeColor::Water),
        "electric" => print!(" {}", PokemonTypeColor::Electric),
        "grass" => print!(" {}", PokemonTypeColor::Grass),
        "ice" => print!(" {}", PokemonTypeColor::Ice),
        "fighting" => print!(" {}", PokemonTypeColor::Fighting),
        "poison" => print!(" {}", PokemonTypeColor::Poison),
        "ground" => print!(" {}", PokemonTypeColor::Ground),
        "flying" => print!(" {}", PokemonTypeColor::Flying),
        "psychic" => print!(" {}", PokemonTypeColor::Psychic),
        "bug" => print!(" {}", PokemonTypeColor::Bug),
        "rock" => print!(" {}", PokemonTypeColor::Rock),
        "ghost" => print!(" {}", PokemonTypeColor::Ghost),
        "dragon" => print!(" {}", PokemonTypeColor::Dragon),
        "dark" => print!(" {}", PokemonTypeColor::Dark),
        "steel" => print!(" {}", PokemonTypeColor::Steel),
        "fairy" => print!(" {}", PokemonTypeColor::Fairy),
        _ => print!(" {}", pokemon),
    }
}

fn handle_pokemon(pokemon: Pokemon) {

    let mut ptypes: Vec<&str> = Vec::new();

    print_basic_info(&pokemon, &mut ptypes);

    println!();

    let type_effectivenes_map = type_effectivenes(ptypes);
    print_type_effectiveness(&type_effectivenes_map);

    println!();

    print_base_status(pokemon);
}

fn print_base_status(pokemon: Pokemon) {
    print!("Stats:");
    println!();
    let mut stat_total = 0;
    for stat_slot in pokemon.stats {
        println!(
            "\t {}: {}, ",
            style(stat_slot.stat_object.name).bold().blue(),
            style(stat_slot.base_stat).bold().red()
        );
        stat_total += stat_slot.base_stat;
    }
    println!("\t total: {}, ", style(stat_total).bold().red());
}

fn print_basic_info<'a>(pokemon: &'a Pokemon, ptypes: &mut Vec<&'a str>) {
    println!("Pokemon: {}", style(&pokemon.name).bold().yellow());
    println!("Id: {}", style(pokemon.id).bold().red());
    println!(
        "Height: {} m",
        style(pokemon.height / 10.0).bold().magenta()
    );
    println!("Weight: {} kg", style(pokemon.weight / 10.0).bold().cyan());
    print!("Types: ");
    for type_slot in &pokemon.types {
        handle_type_color(&type_slot.type_object.name);
        ptypes.push(&type_slot.type_object.name);
    }
}

fn request_pokemon(pokemon_name: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
    let response = reqwest::blocking::get(url)?.json::<Pokemon>()?;
    Ok(response)
}

fn request_pokemon_species(pokemon_name: &str) -> Result<Pokemon, reqwest::Error> {
    // TODO: handle pokemon-species with a specific struct <31-05-23 11:45, erlonbie> //
    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", pokemon_name);
    let response = reqwest::blocking::get(url)?.json::<Pokemon>()?;
    Ok(response)
}


fn type_effectivenes<'a> (ptypes: Vec<&str>) -> HashMap<&'a str, f32> {

    let effectiveness_matrix: [[f32;18];18] = [
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0/2.0, 0.0, 1.0, 1.0, 1.0/2.0, 1.0],
        [1.0, 1.0/2.0, 1.0/2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0/2.0, 1.0, 2.0, 1.0],
        [1.0, 2.0, 1.0/2.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 2.0, 1.0/2.0, 1.0/2.0, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0],
        [1.0, 1.0/2.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0/2.0, 2.0, 1.0/2.0, 1.0, 1.0/2.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0/2.0, 1.0],
        [1.0, 1.0/2.0, 1.0/2.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0],
        [2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0/2.0, 1.0/2.0, 1.0/2.0, 2.0, 0.0, 1.0, 2.0, 2.0, 1.0/2.0],
        [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0/2.0, 1.0/2.0, 1.0, 1.0, 1.0, 1.0/2.0, 1.0/2.0, 1.0, 1.0, 0.0, 2.0],
        [1.0, 2.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 1.0/2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0],
        [1.0, 1.0, 1.0, 1.0/2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0, 1.0, 1.0/2.0, 1.0],
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0/2.0, 1.0],
        [1.0, 1.0/2.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0/2.0, 1.0, 1.0/2.0, 2.0, 1.0, 1.0, 1.0/2.0, 1.0, 2.0, 1.0/2.0, 1.0/2.0],
        [1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0/2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0/2.0, 1.0],
        [0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 0.0],
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0/2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0/2.0, 1.0, 1.0/2.0],
        [1.0, 1.0/2.0, 1.0/2.0, 1.0/2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0/2.0, 2.0],
        [1.0, 1.0/2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0/2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0/2.0, 1.0],
    ];

    let types_list = vec![
            "normal", "fire", "water", "electric", "grass", "ice", "fighting", "poison", "ground",
            "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
        ];

    // Create a hashmap to store the type effectiveness matrix
    let mut type_matrix: HashMap<&str, HashMap<&str, f32>> = HashMap::new();

    // Populate the hashmap with the type effectiveness values
    for (i, attacker) in types_list.iter().enumerate() {
        let mut effectiveness_map: HashMap<&str, f32> = HashMap::new();
        for (j, defender) in types_list.iter().enumerate() {
            effectiveness_map.insert(defender, effectiveness_matrix[j][i]);
        }
        type_matrix.insert(attacker, effectiveness_map);
    }

    let mut neutral_array = [1.0; 18];

    for element in ptypes.iter() {
        let tmp = type_matrix.get(element).expect("Could not create tmp");
        for t in types_list.iter().enumerate() {
            neutral_array[t.0] *= *tmp.get(t.1).expect("Could not multiply in neutral_array");
        }
    }

    let mut final_type_effectivenes: HashMap<&str, f32> = HashMap::new();

    for (i, value) in types_list.iter().enumerate() {
        final_type_effectivenes.insert(value, neutral_array[i]);
    }

    // print_type_effectiveness(&final_type_effectivenes);
    final_type_effectivenes
}

fn print_type_effectiveness(final_type_effectivenes: &HashMap<&str, f32>) {

    let mut weak_to: HashMap<&str, f32> = HashMap::new();
    let mut strong_against: HashMap<&str, f32> = HashMap::new();
    let mut normal_damage_to: HashMap<&str, f32> = HashMap::new();

    final_type_effectivenes.iter().for_each(|(k, v)| {
        if *v > 1.0 {
            weak_to.insert(k, *v);
        } else if *v < 1.0 {
            strong_against.insert(k, *v);
        } else if *v == 1.0 {
            normal_damage_to.insert(k, *v);
        }
    });

    println!("Weak to: ");
    for (k, v) in weak_to.iter() {
        handle_type_color(k.to_lowercase().as_str());
        print!(" {v}x");
    }
    println!();
    println!("Strong against: ");
    for (k, v) in strong_against.iter() {
        handle_type_color(k.to_lowercase().as_str());
        print!(" {v}x");
    }
    println!();
    println!("Normal damage to: ");
    for (k, v) in normal_damage_to.iter() {
        handle_type_color(k.to_lowercase().as_str());
        print!(" {v}x");
    }
}
