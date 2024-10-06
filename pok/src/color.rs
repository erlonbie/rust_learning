use std::fmt;
use termion::{
    color::{self},
    style,
};

struct PokeColor;

impl PokeColor {
    const LAVENDER: color::Rgb = color::Rgb(168, 144, 240);
    const CITRUSORANGE: color::Rgb = color::Rgb(240, 128, 48);
    const CORNFLOWERBLUE: color::Rgb = color::Rgb(104, 144, 240);
    const SAFFRON: color::Rgb = color::Rgb(248, 208, 48);
    const SOFTGREEN: color::Rgb = color::Rgb(120, 200, 80);
    const LIGHTCYAN: color::Rgb = color::Rgb(152, 216, 216);
    const CRIMSON: color::Rgb = color::Rgb(192, 48, 40);
    const PLUM: color::Rgb = color::Rgb(160, 64, 160);
    const SAND: color::Rgb = color::Rgb(224, 192, 104);
    const OLIVE: color::Rgb = color::Rgb(168, 144, 32);
    const WATERMELON: color::Rgb = color::Rgb(248, 88, 136);
    const GOLDENROD: color::Rgb = color::Rgb(168, 184, 32);
    const STONE: color::Rgb = color::Rgb(184, 160, 56);
    const DEEPPURPLE: color::Rgb = color::Rgb(112, 88, 152);
    const ELECTRICINDIGO: color::Rgb = color::Rgb(112, 56, 248);
    const MOCHA: color::Rgb = color::Rgb(112, 88, 72);
    const SILVER: color::Rgb = color::Rgb(184, 184, 208);
    const PASTELPINK: color::Rgb = color::Rgb(238, 153, 172);
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
                write!(
                    f,
                    "{}Normal{}",
                    color::Fg(PokeColor::LAVENDER),
                    style::Reset
                )
            }
            PokemonTypeColor::Fire => {
                write!(
                    f,
                    "{}Fire{}",
                    color::Fg(PokeColor::CITRUSORANGE),
                    style::Reset
                )
            }
            PokemonTypeColor::Water => {
                write!(
                    f,
                    "{}Water{}",
                    color::Fg(PokeColor::CORNFLOWERBLUE),
                    style::Reset
                )
            }
            PokemonTypeColor::Electric => {
                write!(
                    f,
                    "{}Electric{}",
                    color::Fg(PokeColor::SAFFRON),
                    style::Reset
                )
            }
            PokemonTypeColor::Grass => {
                write!(
                    f,
                    "{}Grass{}",
                    color::Fg(PokeColor::SOFTGREEN),
                    style::Reset
                )
            }
            PokemonTypeColor::Ice => {
                write!(f, "{}Ice{}", color::Fg(PokeColor::LIGHTCYAN), style::Reset)
            }
            PokemonTypeColor::Fighting => {
                write!(
                    f,
                    "{}Fighting{}",
                    color::Fg(PokeColor::CRIMSON),
                    style::Reset
                )
            }
            PokemonTypeColor::Poison => {
                write!(f, "{}Poison{}", color::Fg(PokeColor::PLUM), style::Reset)
            }
            PokemonTypeColor::Ground => {
                write!(f, "{}Ground{}", color::Fg(PokeColor::SAND), style::Reset)
            }
            PokemonTypeColor::Flying => {
                write!(f, "{}Flying{}", color::Fg(PokeColor::OLIVE), style::Reset)
            }
            PokemonTypeColor::Psychic => {
                write!(
                    f,
                    "{}Psychic{}",
                    color::Fg(PokeColor::WATERMELON),
                    style::Reset
                )
            }
            PokemonTypeColor::Bug => {
                write!(f, "{}Bug{}", color::Fg(PokeColor::GOLDENROD), style::Reset)
            }
            PokemonTypeColor::Rock => {
                write!(f, "{}Rock{}", color::Fg(PokeColor::STONE), style::Reset)
            }
            PokemonTypeColor::Ghost => {
                write!(
                    f,
                    "{}Ghost{}",
                    color::Fg(PokeColor::DEEPPURPLE),
                    style::Reset
                )
            }
            PokemonTypeColor::Dragon => {
                write!(
                    f,
                    "{}Dragon{}",
                    color::Fg(PokeColor::ELECTRICINDIGO),
                    style::Reset
                )
            }
            PokemonTypeColor::Dark => {
                write!(f, "{}Dark{}", color::Fg(PokeColor::MOCHA), style::Reset)
            }
            PokemonTypeColor::Steel => {
                write!(f, "{}Steel{}", color::Fg(PokeColor::SILVER), style::Reset)
            }
            PokemonTypeColor::Fairy => {
                write!(
                    f,
                    "{}Fairy{}",
                    color::Fg(PokeColor::PASTELPINK),
                    style::Reset
                )
            }
        }
    }
}

pub fn handle_type_color(pokemon: &str) {
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
