enum WeaponType {
    Sword,
    Axe,
    Spear,
}

trait Weapon {
    fn use_weapon(&self);
}

struct Sword;

impl Weapon for Sword {
    fn use_weapon(&self) {
        println!("Swung a sword!");
    }
}

struct Axe;

impl Weapon for Axe {
    fn use_weapon(&self) {
        println!("Chopped with an axe!");
    }
}

struct Spear;

impl Weapon for Spear {
    fn use_weapon(&self) {
        println!("Thrust with a spear!");
    }
}

trait WeaponFactory {
    fn create_weapon(&self, weapon_type: WeaponType) -> Option<Box<dyn Weapon>>;
}

struct Player {
    weapon: Option<Box<dyn Weapon>>,
}

impl Player {
    fn attack(&self) {
        match &self.weapon {
            Some(w) => w.use_weapon(),
            None => println!("Player cannot use this weapon"),
        }
        // self.weapon.use_weapon();
    }
}

struct Knight;

impl WeaponFactory for Knight {
    fn create_weapon(&self, weapon_type: WeaponType) -> Option<Box<dyn Weapon>> {
        match weapon_type {
            WeaponType::Sword => Some(Box::new(Sword)),
            WeaponType::Axe => Some(Box::new(Axe)),
            WeaponType::Spear => Some(Box::new(Spear)),
        }
    }
}

struct Warrior;

impl WeaponFactory for Warrior {
    fn create_weapon(&self, weapon_type: WeaponType) -> Option<Box<dyn Weapon>> {
        match weapon_type {
            WeaponType::Sword => None,
            WeaponType::Axe => Some(Box::new(Axe)),
            WeaponType::Spear => None,
        }
    }
}

fn main() {
    let knight = Knight;
    let weapon = knight.create_weapon(WeaponType::Sword);
    let player = Player { weapon };
    player.attack();

    let warrior = Warrior;
    let weapon2 = warrior.create_weapon(WeaponType::Sword);
    let player2 = Player { weapon: weapon2 };
    player2.attack();
}

