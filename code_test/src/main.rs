use rand::Rng;

//create a pokemon project
struct Pokemon {
    name: String,
    hp: u32,
    attack: u32,
    defense: u32,
    speed: u32,
    level: u32,
    moves: Vec<String>,
}

impl Pokemon {
    fn attack(&self, move_name: &str, enemy: &mut Pokemon) {
        let move_power = match move_name {
            "tackle" => 40,
            "scratch" => 40,
            "ember" => 40,
            "vine whip" => 45,
            "water gun" => 40,
            _ => 0,
        };
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=10);
        let mut critical = 1.0;
        if random_number <= 2 {
            println!("Critical Hit!");
            critical = 1.5;
        }
        let attack = self.attack as f32;
        let defense = enemy.defense as f32;
        let level = self.level as f32;
        let modifier = critical * (random_number as f32 / 100.0) * 1.0;
        let damage = (((2.0 * level + 10.0) / 250.0) * (attack / defense) * move_power as f32
            + 2.0)
            * modifier;
        enemy.hp -= damage as u32;
        println!("{} used {}!", self.name, move_name);
        println!("{} lost {} HP", enemy.name, damage);
        println!("{} has {} HP left", enemy.name, enemy.hp);
    }

    fn new(
        name: &str,
        hp: u32,
        attack: u32,
        defense: u32,
        speed: u32,
        level: u32,
        moves: Vec<String>,
    ) -> Pokemon {
        Pokemon {
            name: name.to_string(),
            hp,
            attack,
            defense,
            speed,
            level,
            moves,
        }
    }

    fn print_moves(&self) {
        println!("Moves:");
        for (i, m) in self.moves.iter().enumerate() {
            println!("{}. {}", i + 1, m);
        }
    }

    fn print_stats(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("HP: {}", self.hp);
        println!("Attack: {}", self.attack);
        println!("Defense: {}", self.defense);
        println!("Speed: {}", self.speed);
    }
}

fn print_battle(p1: &Pokemon, p2: &Pokemon) {
    println!("Battle!");
    println!("{} vs {}", p1.name, p2.name);
    println!("{} has {} HP left", p1.name, p1.hp);
    println!("{} has {} HP left", p2.name, p2.hp);
}

fn main() {
    let _pikachu = Pokemon::new(
        "Pikachu",
        35,
        55,
        40,
        90,
        1,
        vec!["tackle".to_string(), "scratch".to_string()],
    );
    let charmander = Pokemon::new(
        "Charmander",
        39,
        52,
        43,
        65,
        1,
        vec!["scratch".to_string(), "ember".to_string()],
    );
    let _squirtle = Pokemon::new(
        "Squirtle",
        44,
        48,
        65,
        43,
        1,
        vec!["tackle".to_string(), "water gun".to_string()],
    );
    let bulbasaur = Pokemon::new(
        "Bulbasaur",
        45,
        49,
        49,
        45,
        1,
        vec!["tackle".to_string(), "vine whip".to_string()],
    );
    let mut player_pokemon = charmander;
    let mut enemy_pokemon = bulbasaur;
    print_battle(&player_pokemon, &enemy_pokemon);
    println!("------");
    player_pokemon.attack("tackle", &mut enemy_pokemon);
    println!("------");
    enemy_pokemon.attack("tackle", &mut player_pokemon);
    println!("------");
    player_pokemon.attack("scratch", &mut enemy_pokemon);
    println!("------");
    enemy_pokemon.attack("vine whip", &mut player_pokemon);
    println!("------");
}
