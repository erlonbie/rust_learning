#![allow(dead_code)]
#![allow(unused_variables)]

//------------------------------------------------------------------//
//                          Factory Method                          //
//------------------------------------------------------------------//

// trait Hamburgable {
//     fn show_ingredients(&self) -> Vec<Ingredient>;
// }
//
// struct VeggieBurger;
// impl Hamburgable for VeggieBurger {
//     fn show_ingredients(&self) -> Vec<Ingredient> {
//         vec![
//             Ingredient::new("tomato".to_string(), 30),
//             Ingredient::new("lettuce".to_string(), 50),
//             Ingredient::new("nuts".to_string(), 60),
//         ]
//     }
// }
//
// struct BeefBurger;
// impl Hamburgable for BeefBurger {
//     fn show_ingredients(&self) -> Vec<Ingredient> {
//         vec![
//             Ingredient::new("egg".to_string(), 30),
//             Ingredient::new("cheese".to_string(), 50),
//             Ingredient::new("ham".to_string(), 60),
//         ]
//     }
// }
//
// trait HamburgerFactory {
//     fn create_hamburger(&self) -> Box<dyn Hamburgable>;
// }
//
// struct VeggieBurgerFactory;
// impl HamburgerFactory for VeggieBurgerFactory {
//     fn create_hamburger(&self) -> Box<dyn Hamburgable> {
//         Box::new(VeggieBurger)
//     }
// }
//
// struct BeefBurgerFactory;
// impl HamburgerFactory for BeefBurgerFactory {
//     fn create_hamburger(&self) -> Box<dyn Hamburgable> {
//         Box::new(BeefBurger)
//     }
// }
//
// #[derive(Debug)]
// struct Ingredient {
//     name: String,
//     calories: i32,
// }
//
// impl Ingredient {
//     fn new(name: String, calories: i32) -> Self {
//         Self { name, calories }
//     }
// }
//
// fn main() {
//     let veggie_burger_factory = VeggieBurgerFactory;
//     let veggie_burger = veggie_burger_factory.create_hamburger();
//     println!("{:?}", veggie_burger.show_ingredients());
// }

//------------------------------------------------------------------//
//                             Builder                              //
//------------------------------------------------------------------//

// #[derive(Debug, Clone, Copy)]
// enum HamburgerComponent {
//     Bun(TypeBun),
//     Patty(TypePatty),
//     Cheese(TypeCheese),
// }
//
// #[derive(Debug, Clone, Copy)]
// enum TypeBun {
//     Brioche,
//     Potato,
// }
// #[derive(Debug, Clone, Copy)]
// enum TypePatty {
//     Vegan,
//     NonVegan,
// }
// #[derive(Debug, Clone, Copy)]
// enum TypeCheese {
//     Chedar,
//     Muzzarela,
// }
//
// #[derive(Debug, Clone, Copy)]
// struct Hamburger {
//     bun: Option<TypeBun>,
//     patty: Option<TypePatty>,
//     cheese: Option<TypeCheese>,
// }
//
// impl Hamburger {
//     fn set_bun(&mut self, bun: TypeBun) {
//         self.bun = Some(bun);
//     }
//     fn set_patty(&mut self, patty: TypePatty) {
//         self.patty = Some(patty);
//     }
//
//     fn set_cheese(&mut self, cheese: TypeCheese) {
//         self.cheese = Some(cheese);
//     }
//
//     fn default() -> Self {
//         Self {
//             bun: None,
//             patty: None,
//             cheese: None,
//         }
//     }
//
//     fn show_burger(&self) -> Vec<String> {
//         let mut ingredients = Vec::new();
//
//         if self.bun.is_some() {
//             match self.bun.unwrap() {
//                 TypeBun::Brioche => ingredients.push("Brioche".to_string()),
//                 TypeBun::Potato => ingredients.push("Potato".to_string()),
//             }
//         }
//
//         if self.patty.is_some() {
//             match self.patty.unwrap() {
//                 TypePatty::Vegan => ingredients.push("Vegan".to_string()),
//                 TypePatty::NonVegan => ingredients.push("NonVegan".to_string()),
//             }
//         }
//
//         if self.cheese.is_some() {
//             match self.cheese.unwrap() {
//                 TypeCheese::Chedar => ingredients.push("Chedar".to_string()),
//                 TypeCheese::Muzzarela => ingredients.push("Muzzarela".to_string()),
//             }
//         }
//
//         ingredients
//     }
// }
//
// struct HamburgerBuilder {
//     hamburger: Hamburger,
// }
//
// impl HamburgerBuilder {
//     fn new() -> Self {
//         Self {
//             hamburger: Hamburger::default(),
//         }
//     }
//
//     fn add(&mut self, ingredient: HamburgerComponent) -> &mut Self {
//         match ingredient {
//             HamburgerComponent::Bun(bun) => match bun {
//                 TypeBun::Brioche => {
//                     self.hamburger.set_bun(TypeBun::Brioche);
//                     self
//                 }
//                 TypeBun::Potato => {
//                     self.hamburger.set_bun(TypeBun::Potato);
//                     self
//                 }
//             },
//             HamburgerComponent::Patty(patty) => match patty {
//                 TypePatty::Vegan => {
//                     self.hamburger.set_patty(TypePatty::Vegan);
//                     self
//                 }
//                 TypePatty::NonVegan => {
//                     self.hamburger.set_patty(TypePatty::NonVegan);
//                     self
//                 }
//             },
//             HamburgerComponent::Cheese(cheese) => match cheese {
//                 TypeCheese::Chedar => {
//                     self.hamburger.set_cheese(TypeCheese::Chedar);
//                     self
//                 }
//                 TypeCheese::Muzzarela => {
//                     self.hamburger.set_cheese(TypeCheese::Muzzarela);
//                     self
//                 }
//             },
//         }
//     }
//
//     fn build(&mut self) -> Hamburger {
//         self.hamburger
//     }
// }
//
// pub fn main() {
//     let burger1 = HamburgerBuilder::new()
//         .add(HamburgerComponent::Bun(TypeBun::Brioche))
//         .add(HamburgerComponent::Patty(TypePatty::NonVegan))
//         .add(HamburgerComponent::Cheese(TypeCheese::Chedar))
//         .build();
//
//     println!("{burger1:?}");
//     println!("{:?}", burger1.show_burger());
// }

//------------------------------------------------------------------//
//                            Builder 2                             //
//------------------------------------------------------------------//

#[derive(Debug, Clone, Copy)]
enum HamburgerComponent {
    Bun(TypeBun),
    Patty(TypePatty),
    Cheese(TypeCheese),
}

#[derive(Debug, Clone, Copy)]
enum TypeBun {
    Brioche,
    Potato,
}
#[derive(Debug, Clone, Copy)]
enum TypePatty {
    Vegan,
    NonVegan,
}
#[derive(Debug, Clone, Copy)]
enum TypeCheese {
    Chedar,
    Muzzarela,
}

#[derive(Debug, Clone, Copy)]
struct Hamburger {
    bun: Option<TypeBun>,
    patty: Option<TypePatty>,
    cheese: Option<TypeCheese>,
}

impl Hamburger {
    fn set_bun(&mut self, bun: TypeBun) {
        self.bun = Some(bun);
    }
    fn set_patty(&mut self, patty: TypePatty) {
        self.patty = Some(patty);
    }

    fn set_cheese(&mut self, cheese: TypeCheese) {
        self.cheese = Some(cheese);
    }

    fn default() -> Self {
        Self {
            bun: None,
            patty: None,
            cheese: None,
        }
    }

    fn show_burger(self) -> Vec<String> {
        let mut ingredients = Vec::new();

        if self.bun.is_some() {
            match self.bun.unwrap() {
                TypeBun::Brioche => ingredients.push("Brioche".to_string()),
                TypeBun::Potato => ingredients.push("Potato".to_string()),
            }
        }

        if self.patty.is_some() {
            match self.patty.unwrap() {
                TypePatty::Vegan => ingredients.push("Vegan".to_string()),
                TypePatty::NonVegan => ingredients.push("NonVegan".to_string()),
            }
        }

        if self.cheese.is_some() {
            match self.cheese.unwrap() {
                TypeCheese::Chedar => ingredients.push("Chedar".to_string()),
                TypeCheese::Muzzarela => ingredients.push("Muzzarela".to_string()),
            }
        }

        ingredients
    }
}

struct HamburgerBuilder {
    hamburger: Hamburger,
}

impl HamburgerBuilder {
    fn new() -> Self {
        Self {
            hamburger: Hamburger::default(),
        }
    }

    fn add(mut self, ingredient: HamburgerComponent) -> Self {
        match ingredient {
            HamburgerComponent::Bun(bun) => match bun {
                TypeBun::Brioche => {
                    self.hamburger.set_bun(TypeBun::Brioche);
                    self
                }
                TypeBun::Potato => {
                    self.hamburger.set_bun(TypeBun::Potato);
                    self
                }
            },
            HamburgerComponent::Patty(patty) => match patty {
                TypePatty::Vegan => {
                    self.hamburger.set_patty(TypePatty::Vegan);
                    self
                }
                TypePatty::NonVegan => {
                    self.hamburger.set_patty(TypePatty::NonVegan);
                    self
                }
            },
            HamburgerComponent::Cheese(cheese) => match cheese {
                TypeCheese::Chedar => {
                    self.hamburger.set_cheese(TypeCheese::Chedar);
                    self
                }
                TypeCheese::Muzzarela => {
                    self.hamburger.set_cheese(TypeCheese::Muzzarela);
                    self
                }
            },
        }
    }

    fn build(self) -> Hamburger {
        self.hamburger
    }
}

pub fn main() {
    let burger1 = HamburgerBuilder::new()
        .add(HamburgerComponent::Bun(TypeBun::Brioche))
        .add(HamburgerComponent::Patty(TypePatty::NonVegan))
        .add(HamburgerComponent::Cheese(TypeCheese::Chedar))
        .build();

    println!("{burger1:?}");
    println!("{:?}", burger1.show_burger());
}

//------------------------------------------------------------------//
//                            Typestate                             //
//------------------------------------------------------------------//

// #[derive(Debug, Clone, Copy)]
// enum HamburgerComponent {
//     Bun(TypeBun),
//     Patty(TypePatty),
//     Cheese(TypeCheese),
// }
//
// #[derive(Debug, Clone, Copy)]
// enum TypeBun {
//     Brioche,
//     Potato,
// }
// #[derive(Debug, Clone, Copy)]
// enum TypePatty {
//     Vegan,
//     NonVegan,
// }
// #[derive(Debug, Clone, Copy)]
// enum TypeCheese {
//     Chedar,
//     Muzzarela,
// }
//
// #[derive(Default, Debug, Clone, Copy)]
// struct Hamburger {
//     bun: Option<TypeBun>,
//     patty: Option<TypePatty>,
//     cheese: Option<TypeCheese>,
// }
//
// impl Hamburger {
//     fn set_bun(&mut self, bun: TypeBun) {
//         self.bun = Some(bun);
//     }
//     fn set_patty(&mut self, patty: TypePatty) {
//         self.patty = Some(patty);
//     }
//
//     fn set_cheese(&mut self, cheese: TypeCheese) {
//         self.cheese = Some(cheese);
//     }
// }
//
// #[derive(Default, Clone)]
// struct NoCity;
//
// #[derive(Default, Clone)]
// struct City(String);
//
// #[derive(Default, Clone)]
// struct NoRestaurant;
//
// #[derive(Default, Clone)]
// struct Restaurant(String);
//
// #[derive(Default)]
// struct HamburgerBuilder<R, C> {
//     restaraut: R,
//     city: C,
//     hamburger: Hamburger,
// }
//
// impl HamburgerBuilder<Restaurant, City> {
//     fn new() -> Self {
//         Self::default()
//     }
// }
//
// impl HamburgerBuilder<Restaurant, City> {
//     fn build(&mut self) -> Hamburger {
//         self.hamburger
//     }
//
// }
//
// impl<R, C> HamburgerBuilder<R, C> {
//
//     fn restaraut(&mut self, restaraut: String) -> HamburgerBuilder<Restaurant, C> {
//         let Self {
//             city,
//             hamburger,
//             ..
//         } = self;
//         HamburgerBuilder {
//             restaraut: Restaurant(restaraut),
//             city,
//             hamburger: *hamburger
//         }
//     }
//
//     fn add(&mut self, ingredient: HamburgerComponent) -> &mut Self {
//         match ingredient {
//             HamburgerComponent::Bun(bun) => match bun {
//                 TypeBun::Brioche => {
//                     self.hamburger.set_bun(TypeBun::Brioche);
//                     self
//                 }
//                 TypeBun::Potato => {
//                     self.hamburger.set_bun(TypeBun::Potato);
//                     self
//                 }
//             },
//             HamburgerComponent::Patty(patty) => match patty {
//                 TypePatty::Vegan => {
//                     self.hamburger.set_patty(TypePatty::Vegan);
//                     self
//                 }
//                 TypePatty::NonVegan => {
//                     self.hamburger.set_patty(TypePatty::NonVegan);
//                     self
//                 }
//             },
//             HamburgerComponent::Cheese(cheese) => match cheese {
//                 TypeCheese::Chedar => {
//                     self.hamburger.set_cheese(TypeCheese::Chedar);
//                     self
//                 }
//                 TypeCheese::Muzzarela => {
//                     self.hamburger.set_cheese(TypeCheese::Muzzarela);
//                     self
//                 }
//             },
//         }
//     }
//
// }
//
// pub fn main() {
//     let burger1 = HamburgerBuilder::new()
//         .add(HamburgerComponent::Bun(TypeBun::Brioche))
//         .add(HamburgerComponent::Patty(TypePatty::NonVegan))
//         .add(HamburgerComponent::Cheese(TypeCheese::Chedar))
//         .build();
//
//     println!("{burger1:#?}");
//     println!("{:#?}", burger1.show_burger());
// }
