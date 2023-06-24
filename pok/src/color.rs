use std::fmt;
use termion::{color, style};

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
                    color::Fg(color::Rgb(168, 144, 240)),
                    style::Reset
                )
            }
            PokemonTypeColor::Fire => {
                write!(
                    f,
                    "{}Fire{}",
                    color::Fg(color::Rgb(240, 128, 48)),
                    style::Reset
                )
            }
            PokemonTypeColor::Water => {
                write!(
                    f,
                    "{}Water{}",
                    color::Fg(color::Rgb(104, 144, 240)),
                    style::Reset
                )
            }
            PokemonTypeColor::Electric => {
                write!(
                    f,
                    "{}Electric{}",
                    color::Fg(color::Rgb(248, 208, 48)),
                    style::Reset
                )
            }
            PokemonTypeColor::Grass => {
                write!(
                    f,
                    "{}Grass{}",
                    color::Fg(color::Rgb(120, 200, 80)),
                    style::Reset
                )
            }
            PokemonTypeColor::Ice => {
                write!(
                    f,
                    "{}Ice{}",
                    color::Fg(color::Rgb(152, 216, 216)),
                    style::Reset
                )
            }
            PokemonTypeColor::Fighting => {
                write!(
                    f,
                    "{}Fighting{}",
                    color::Fg(color::Rgb(192, 48, 40)),
                    style::Reset
                )
            }
            PokemonTypeColor::Poison => {
                write!(
                    f,
                    "{}Poison{}",
                    color::Fg(color::Rgb(160, 64, 160)),
                    style::Reset
                )
            }
            PokemonTypeColor::Ground => {
                write!(
                    f,
                    "{}Ground{}",
                    color::Fg(color::Rgb(224, 192, 104)),
                    style::Reset
                )
            }
            PokemonTypeColor::Flying => {
                write!(
                    f,
                    "{}Flying{}",
                    color::Fg(color::Rgb(168, 144, 240)),
                    style::Reset
                )
            }
            PokemonTypeColor::Psychic => {
                write!(
                    f,
                    "{}Psychic{}",
                    color::Fg(color::Rgb(248, 88, 136)),
                    style::Reset
                )
            }
            PokemonTypeColor::Bug => {
                write!(
                    f,
                    "{}Bug{}",
                    color::Fg(color::Rgb(168, 184, 32)),
                    style::Reset
                )
            }
            PokemonTypeColor::Rock => {
                write!(
                    f,
                    "{}Rock{}",
                    color::Fg(color::Rgb(184, 160, 56)),
                    style::Reset
                )
            }
            PokemonTypeColor::Ghost => {
                write!(
                    f,
                    "{}Ghost{}",
                    color::Fg(color::Rgb(112, 88, 152)),
                    style::Reset
                )
            }
            PokemonTypeColor::Dragon => {
                write!(
                    f,
                    "{}Dragon{}",
                    color::Fg(color::Rgb(112, 56, 248)),
                    style::Reset
                )
            }
            PokemonTypeColor::Dark => {
                write!(
                    f,
                    "{}Dark{}",
                    color::Fg(color::Rgb(112, 88, 72)),
                    style::Reset
                )
            }
            PokemonTypeColor::Steel => {
                write!(
                    f,
                    "{}Steel{}",
                    color::Fg(color::Rgb(184, 184, 208)),
                    style::Reset
                )
            }
            PokemonTypeColor::Fairy => {
                write!(
                    f,
                    "{}Fairy{}",
                    color::Fg(color::Rgb(238, 153, 172)),
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
