use crate::{utils::handle_pokemon, models::Pokemon};

pub fn get_pokemon(pokemon_name: &str) {
    match request_pokemon(pokemon_name) {
        Ok(pokemon) => handle_pokemon(pokemon),
        Err(_) => match request_pokemon_species(pokemon_name) {
            // BUG: this is not working yet <31-05-23 11:46, erlonbie> //
            Ok(pokemon) => handle_pokemon(pokemon),
            Err(e) => println!("Pokemon not found: {}", e),
        },
    }
}

pub fn request_pokemon(pokemon_name: &str) -> Result<Pokemon, reqwest::Error> {
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
