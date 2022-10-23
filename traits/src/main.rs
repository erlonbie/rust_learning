struct Person {
    name: String,
}

struct Cat {
    name: String,
}

struct Rabbit {
    name: String,
}

pub trait Eat {
    fn eat_dinner(&self) {
        println!("I eat from a dish");
    }
}

impl Eat for Person {
    fn eat_dinner(&self) {
        println!("I eat from a plate");
    }
}

impl Eat for Cat {
    fn eat_dinner(&self) {
        println!("I eat from a cat bowl");
    }
}

impl Eat for Rabbit {
    
}

fn main() {
    let person = Person {
        name: "Erlon".to_string(),
    };
    person.eat_dinner();

    let cat = Cat {
        name: "Vivi".to_string()
    };
    cat.eat_dinner();

    let rabbit = Rabbit {
        name: "Coelho".to_string()
    };
    rabbit.eat_dinner();
}
