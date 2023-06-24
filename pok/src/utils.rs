use crate::print::{print_base_status, print_basic_info, print_type_effectiveness};
use std::collections::HashMap;
use crate::models::Pokemon;

pub fn handle_pokemon(pokemon: Pokemon) {
    let mut ptypes: Vec<&str> = Vec::new();

    print_basic_info(&pokemon, &mut ptypes);

    println!();

    let type_effectivenes_map = type_effectivenes(ptypes);
    print_type_effectiveness(&type_effectivenes_map);

    println!();

    print_base_status(pokemon);
}

pub fn type_effectivenes<'a>(ptypes: Vec<&str>) -> HashMap<&'a str, f32> {
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

    final_type_effectivenes
}
