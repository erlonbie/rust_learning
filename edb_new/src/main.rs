use std::collections::{BTreeMap, HashMap};

use edgedb_tokio::{Client, Queryable};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio;
use serde_json;

#[derive(Debug, Serialize, Queryable)]
pub struct Pokemon {
    pub poke_id: i16,
    pub name: String,
    pub height: f64,
    pub weight: f64,
    pub types: Vec<String>,
    pub weak_to: Vec<(String, f32)>,
    pub strong_against: Vec<(String, f32)>,
    pub normal_damage_to: Vec<(String, f32)>,
    pub hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub special_attack: i16,
    pub special_defense: i16,
    pub speed: i16,
}

impl Pokemon {
    fn into_json(&self) -> String {
        serde_json::json!({
            "poke_id": self.poke_id,
            "name": self.name,
            "height": self.height,
            "weight": self.weight,
            "types": self.types,
            "weak_to": self.weak_to,
            "strong_against": self.strong_against,
            "normal_damage_to": self.normal_damage_to,
            "hp": self.hp,
            "attack": self.attack,
            "defense": self.defense,
            "special_attack": self.special_attack,
            "special_defense": self.special_defense,
            "speed": self.speed,
        }).to_string()
    }
} 

#[derive(Deserialize, Serialize)]
struct PokeApiResponse {
    id: i32,
    name: String,
    height: i32, // decimeters -> convert to meters
    weight: i32, // hectograms -> convert to kilograms
    stats: Vec<PokemonStat>,
    types: Vec<PokemonTypeSlot>,
}

#[derive(Deserialize, Serialize)]
struct PokemonTypeSlot {
    #[serde(rename = "type")]
    type_name: PokemonType,
}

#[derive(Deserialize, Serialize)]
struct PokemonType {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct PokemonStat {
    base_stat: i32,
    stat: Stat,
}

#[derive(Deserialize, Serialize)]
struct Stat {
    name: String,
}

async fn fetch_pokemon(id: i32) -> Result<PokeApiResponse, Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let response = reqwest::get(&url).await?;
    response.json::<PokeApiResponse>().await
}

async fn insert_pokemon(client: &Client, pokemon: PokeApiResponse, id: i16) {
    let height = pokemon.height as f64 / 10.0;
    let weight = pokemon.weight as f64 / 10.0;

    let mut ptypes: Vec<&str> = Vec::new();
    for type_slot in &pokemon.types {
        ptypes.push(&type_slot.type_name.name);
    }

    let type_effectivenes_map = type_effectivenes(&ptypes);

    let mut weak_to: BTreeMap<&str, f32> = BTreeMap::new();
    let mut strong_against: BTreeMap<&str, f32> = BTreeMap::new();
    let mut normal_damage_to: BTreeMap<&str, f32> = BTreeMap::new();

    type_effectivenes_map.iter().for_each(|(k, v)| {
        if *v > 1.0 {
            weak_to.insert(k, *v);
        } else if *v < 1.0 {
            strong_against.insert(k, *v);
        } else if *v == 1.0 {
            normal_damage_to.insert(k, *v);
        }
    });

    let mut stats_map = std::collections::HashMap::new();
    for stat in pokemon.stats {
        stats_map.insert(stat.stat.name.clone(), stat.base_stat);
    }

    let name = &pokemon.name;
    let hp = *stats_map.get("hp").unwrap_or(&0) as i16;
    let attack = *stats_map.get("attack").unwrap_or(&0) as i16;
    let defense = *stats_map.get("defense").unwrap_or(&0) as i16;
    let special_attack = *stats_map.get("special-attack").unwrap_or(&0) as i16;
    let special_defense = *stats_map.get("special-defense").unwrap_or(&0) as i16;
    let speed = *stats_map.get("speed").unwrap_or(&0) as i16;

    let pokemon_data = Pokemon {
        poke_id: id,
        name: name.clone(),
        height,
        weight,
        types: ptypes.iter().map(|x| x.to_string()).collect(),
        weak_to: weak_to.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        strong_against: strong_against.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        normal_damage_to: normal_damage_to.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
    };

    // let insert_data = vec![&pokemon_data];
    // let insert_data_str = serde_json::to_string(&insert_data).unwrap();
    let as_value = pokemon_data.into_json();



    // println!("{:?}", insert_data_str);

    let query = r#"
        WITH pokemon := to_json(<str>$0),
        INSERT Pokemon {
            poke_id := <int16>json_get(pokemon,'poke_id'),
            name := <str>json_get(pokemon,'name'),
            height := <float64>json_get(pokemon,'height'),
            weight := <float64>json_get(pokemon,'weight'),
            types := array_unpack(<array<str>>json_get(pokemon,'types')),
            weak_to := array_unpack(<array<tuple<str, float32>>>json_get(pokemon,'weak_to')),
            strong_against := array_unpack(<array<tuple<str, float32>>>json_get(pokemon,'strong_against')),
            normal_damage_to := array_unpack(<array<tuple<str, float32>>>json_get(pokemon,'normal_damage_to')),
            hp := <int16>json_get(pokemon,'hp'),
            attack := <int16>json_get(pokemon,'attack'),
            defense := <int16>json_get(pokemon,'defense'),
            special_attack := <int16>json_get(pokemon,'special_attack'),
            special_defense := <int16>json_get(pokemon,'special_defense'),
            speed := <int16>json_get(pokemon,'speed')
        }

    "#;

    // let _insert_as_value: Vec<Pokemon> = client.query(query, &(as_value,)).await.unwrap();

    client.execute(query, &(as_value,)).await.unwrap();
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = edgedb_tokio::create_client().await?;
//
//     for id in 0..=150 {
//         match fetch_pokemon(id).await {
//             Ok(pokemon) => {
//                 println!("Fetched: {} (ID: {})", pokemon.name, id);
//                 insert_pokemon(&client, pokemon).await;
//             }
//             Err(e) => {
//                 eprintln!("Failed to fetch or insert Pokemon ID {}: {}", id, e);
//             }
//         }
//     }
//
//     Ok(())
// }


use tokio::task;
// rustc: move occurs because `client` has type `edgedb_tokio::Client`, which does not implement the `Copy` trait [E0382] rustc: use of moved value: `client` value moved here, in previous iteration of loop [E0382]


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = edgedb_tokio::create_client().await?;

    let mut tasks = vec![];
    for id in 0..=151 {
        let task = task::spawn(async move {
            match fetch_pokemon(id).await {
                Ok(pokemon) => {
                    let client = edgedb_tokio::create_client().await.unwrap();
                    println!("Fetched: {} (ID: {})", pokemon.name, id);
                    insert_pokemon(&client, pokemon, id as i16).await;
                }
                Err(e) => {
                    eprintln!("Failed to fetch or insert Pokemon ID {}: {}", id, e);
                }
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

pub fn type_effectivenes<'a>(ptypes: &Vec<&str>) -> HashMap<&'a str, f32> {
    let effectiveness_matrix: [[f32; 18]; 18] = [
        [ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 0.0, 1.0, 1.0, 1.0 / 2.0, 1.0, ],
        [ 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 1.0, 2.0, 1.0, ],
        [ 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, ],
        [ 1.0, 1.0, 2.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, ],
        [ 1.0, 1.0 / 2.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0 / 2.0, 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 1.0, ],
        [ 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0, ],
        [ 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0, 2.0, 0.0, 1.0, 2.0, 2.0, 1.0 / 2.0, ],
        [ 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0, 0.0, 2.0, ],
        [ 1.0, 2.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 1.0 / 2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, ],
        [ 1.0, 1.0, 1.0, 1.0 / 2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 1.0, ],
        [ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0 / 2.0, 1.0, ],
        [ 1.0, 1.0 / 2.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 2.0, 1.0, 1.0, 1.0 / 2.0, 1.0, 2.0, 1.0 / 2.0, 1.0 / 2.0, ],
        [ 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 1.0, ],
        [ 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0, ],
        [ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 0.0, ],
        [ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0 / 2.0, 1.0, 1.0 / 2.0, ],
        [ 1.0, 1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0 / 2.0, 2.0, ],
        [ 1.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0 / 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0 / 2.0, 1.0, ],
    ];

    let types_list = [
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

    final_type_effectivenes
}
