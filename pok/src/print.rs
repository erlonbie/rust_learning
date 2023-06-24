use crate::color::handle_type_color;
use crate::models::Pokemon;
use console::style;
use std::collections::HashMap;

pub fn print_base_status(pokemon: Pokemon) {
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

pub fn print_basic_info<'a>(pokemon: &'a Pokemon, ptypes: &mut Vec<&'a str>) {
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

pub fn print_type_effectiveness(final_type_effectivenes: &HashMap<&str, f32>) {
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
