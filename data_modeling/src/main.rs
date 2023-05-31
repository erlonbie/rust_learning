//------------------------------------------------------------------//
//                              Part 1                              //
//------------------------------------------------------------------//

// enum RealCat {
//     Alive { hungry: bool },
//     Dead,
// }
//
// fn main() {
//     let realcat = RealCat::Alive { hungry: true };
//     match realcat {
//         RealCat::Alive { hungry } if hungry => {
//             println!("The cat is alive and hungry!");
//         }
//         RealCat::Alive { hungry } => {
//             println!("The cat is alive and not hungry!");
//         }
//         RealCat::Dead => println!("The cat is dead."),
//     }
// }

//------------------------------------------------------------------//
//                              Part 2                              //
//------------------------------------------------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Transitions {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Player {
    state: State,
}

impl Player {
    fn new() -> Self {
        Self {
            state: State::Mario,
        }
    }

    fn collect(&mut self, power: Transitions) {
        match (&self.state, power) {
            (State::Mario, Transitions::Mushroom) => self.state = State::SuperMario,
            (_, Transitions::Flower) => self.state = State::FireMario,
            (_, Transitions::Feather) => self.state = State::CapeMario,
            (_, Transitions::Mushroom) => {}
        }
    }
}

pub fn main() {
    let mut istme = Player::new();
    println!("{:?}", istme.state);
    istme.collect(Transitions::Feather);
    println!("{:?}", istme.state);
    istme.collect(Transitions::Mushroom);
    println!("{:?}", istme.state);
    istme.collect(Transitions::Flower);
    println!("{:?}", istme.state);
    istme.collect(Transitions::Mushroom);
    println!("{:?}", istme.state);
}
