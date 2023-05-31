use std::collections::HashMap;

fn main() {

    let effectiveness_matrix2: [[f32;18];18] = [
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

    let types2 = vec![
            "Normal", "Fire", "Water", "Electric", "Grass", "Ice", "Fighting", "Poison", "Ground",
            "Flying", "Psychic", "Bug", "Rock", "Ghost", "Dragon", "Dark", "Steel", "Fairy",
        ];

    // Create a hashmap to store the type effectiveness matrix
    let mut type_matrix: HashMap<&str, HashMap<&str, f32>> = HashMap::new();

    // Populate the hashmap with the type effectiveness values
    for (i, attacker) in types2.iter().enumerate() {
        let mut effectiveness_map: HashMap<&str, f32> = HashMap::new();
        for (j, defender) in types2.iter().enumerate() {
            effectiveness_map.insert(defender, effectiveness_matrix2[j][i]);
        }
        type_matrix.insert(attacker, effectiveness_map);
    }

    // Test the type effectiveness matrix
    let attacker = "Flying";
    let defender = "Ghost";
    if let Some(effectiveness) = type_matrix.get(attacker).and_then(|entry| entry.get(defender)) {
        println!("Type effectiveness: {}x", effectiveness);
    } else {
        println!("Type effectiveness not found.");
    }


    let attacker_list = vec!["Rock", "Dark"];

    println!("{types2:?}");
    let mut neutral_array = [1.0; 18];

    for element in attacker_list.iter() {
        let tmp = type_matrix.get(element).unwrap();
        println!("for: {:?}", element);
        println!("neutral_array: {:?}", neutral_array);
        for t in types2.iter().enumerate() {
            neutral_array[t.0] *= *tmp.get(t.1).unwrap();
            // println!("\t {}: {}", t.1, tmp.get(t.1).unwrap());
        }
        println!("neutral_array: {:?}", neutral_array);
    }

    let mut final_type_effectivenes: HashMap<&str, f32> = HashMap::new();

    for (i, value) in types2.iter().enumerate() {
        final_type_effectivenes.insert(value, neutral_array[i]);
    }

    println!("{:?}", final_type_effectivenes);

}

// fn capitalize_first_letter(words: Vec<&str>) -> Vec<&str>  {
//     let w = words
//         .iter()
//         .map(|&word| {
//             let mut chars = word.chars();
//             match chars.next() {
//                 None => word,
//                 Some(first) => {
//                     let capitalized = first.to_uppercase();
//                     let mut new_word = capitalized.to_string();
//                     new_word.push_str(chars.as_str());
//                     Box::leak(new_word.into_boxed_str())
//                 }
//             }
//         })
//         .collect::<Vec<&str>>();
//
//     w
//     // type_effectivenes(w);
// }
