#[macro_use]
extern crate diesel;

use diesel::prelude::{Connection, Insertable, Queryable, RunQueryDsl};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use serde::Deserialize;
use tokio;

// Definição do modelo das tabelas
#[derive(Queryable, Insertable)]
#[table_name = "pokemon"]
struct Pokemon {
    id: Option<i32>,
    name: String,
    height: i32,
    weight: i32,
}

// Definição do arquivo schema.rs gerado pelo diesel
pub mod schema {
    table! {
        pokemon (id) {
            id -> Integer,
            name -> Text,
            height -> Integer,
            weight -> Integer,
        }
    }
}

use self::schema::pokemon;

#[derive(Deserialize)]
struct PokemonAPI {
    name: String,
    height: i32,
    weight: i32,
}

async fn fetch_pokemon_data(pokemon_name: &str) -> Result<PokemonAPI, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
    let response = reqwest::get(&url).await?;
    let pokemon: PokemonAPI = response.json().await?;
    Ok(pokemon)
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();
    let pokemon_name = "squirtle";

    match fetch_pokemon_data(pokemon_name).await {
        Ok(pokemon_api) => {
            let new_pokemon = Pokemon {
                id: None,
                name: pokemon_api.name,
                height: pokemon_api.height,
                weight: pokemon_api.weight,
            };

            diesel::insert_into(pokemon::table)
                .values(&new_pokemon)
                .execute(&mut connection)?;

            println!("Data for Pokémon {} saved to the database.", new_pokemon.name);
        }
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
        }
    }
    Ok(())
}

