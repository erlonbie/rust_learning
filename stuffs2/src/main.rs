// use std::collections::HashMap;
//
// fn re(v: Vec<i32>) -> Vec<i32> {
//     println!("re - {} {}", v[0], v[1]);
//     v
// }
//
// fn borrow1(v: &[i32]) {
//     println!("borrow1 - {} {}", v[0], v[1]);
// }
//
// fn borrow2(v: &[i32]) {
//     println!("borrow2 - {}", v[0] + v[1]);
// }
//
// fn main() {
//    let mut my_vec = Vec::new();
//    for i in 1..10 {
//        my_vec.push(i);
//    }
//
//    my_vec = re(my_vec);
//    println!("after re - {} {}", my_vec[0], my_vec[1]);
//    borrow1(&my_vec);
//    println!("after borrow1 - {} {}", &my_vec[0], &my_vec[1]);
//    borrow2(&my_vec);
//    println!("after borrow2 - {} {}", my_vec[0], my_vec[1]);
//
//    let ar = [0;5];
//    println!("{:?}", ar);
//
//    let slice = &ar[1..3];
//    println!("{:?}", slice);
//
//    let mut subs = HashMap::new();
//    subs.insert(String::from("LGR"), 10);
//    subs.entry("LGR".to_owned()).or_insert(3);
//    println!("{:?}", subs);
//
//    let vv = [4..9];
//    println!("{:?}",vv);
//
// }

// #![allow(dead_code)] // esta linha evita avisos do compilador
//
// enum Species { Crab, Octopus, Fish, Clam }
// enum PoisonType { Acidic, Painful, Lethal }
// enum Size { Big, Small }
// enum Weapon {
//     Claw(i32, Size),
//     Poison(PoisonType),
// }
//
// struct SeaCreature {
//     species: Species,
//     name: String,
//     arms: i32,
//     legs: i32,
//     weapon: Weapon,
// }
//
// fn main() {
//     // os dados de SeaCreature estão na pilha
//     let ferris = SeaCreature {
//         // A struct da String também está na pilha,
//         // mas mantém uma referência dos dados na heap
//         species: Species::Crab,
//         name: String::from("Ferris"),
//         arms: 2,
//         legs: 4,
//         weapon: Weapon::Claw(2, Size::Small),
//     };
//
//     match ferris.species {
//         Species::Crab => {
//             match ferris.weapon {
//                 Weapon::Claw(num_claws,size) => {
//                     let size_description = match size {
//                         Size::Big => "grandes",
//                         Size::Small => "pequenas"
//                     };
//                     let n_claw = match num_claws {
//                         2 => "dois",
//                         _ => "mais de dois"
//                     };
//                     println!("Ferris é um caranguejo com {} garras {}", n_claw, size_description)
//                 },
//                 _ => println!("Ferris é um caranguejo com outro tipo de arma")
//             }
//         },
//         _ => println!("Ferris é outro tipo de animal"),
//     }
// }

// fn main() -> Result<(), String> {
//     let result = can_fail(41).expect("Emergency panic");
//     // match result {
//     //     Ok(v) => println!("YAY {}", v),
//     //     Err(e) => println!("ops {}", e)
//     // }
//     println!("encontrei = {:?}", result);
//     Ok(())
// }
//
// fn can_fail(arg: i32) -> Result<f32, String> {
//     if arg == 42 {
//         Ok(14.0)
//     }
//     else {
//         Err(String::from("didn't work"))
//     }
// }

// pub fn main() {
//     let mut i32_vec = Vec::new();
//     println!("{:?}",i32_vec);
//     i32_vec.push(1);
//     i32_vec.push(2);
//     i32_vec.push(3);
//     println!("{:?}",i32_vec);
//
//     let str_vec = vec![String::from("Apple - 🍎"), String::from("Banna - 🍌"), String::from("Pineapple - 🍍")];
//     for word in str_vec.iter() {
//        println!("{}", word);
//     }
// }

// struct Foo {
//     x: i32,
// }
//
// fn do_something(f: Foo) {
//     println!("{}", f.x);
//     // f is dropped here
// }
//
// fn main() {
//     let mut foo = Foo { x: 42 };
//     let f = &mut foo;
//
//     // FAILURE: do_something(foo) would fail because
//     // foo cannot be moved while mutably borrowed
//
//     // FAILURE: foo.x = 13; would fail here because
//     // foo is not modifiable while mutably borrowed
//
//     f.x = 13;
//     // f is dropped here because it's no longer used after this point
//
//     println!("{}", foo.x);
//
//     // this works now because all mutable references were dropped
//     foo.x = 7;
//
//     // move foo's ownership to a function
//     do_something(foo);
// }

// fn main() {
//     let mut foo = 42;
//     let f = &mut foo;
//     let bar = *f; // get a copy of the owner's value
//     *f = 13;      // set the reference's owner's value
//     println!("{}", bar);
//     println!("{}", f);
//     println!("{}", foo);
// }

// #[derive(Debug)]
// struct Point {
//     width: i32,
//     height: i32,
// }
//
// #[derive(Debug)]
// struct Circle {
//     radious: i32
// }
//
// #[derive(Debug)]
// enum Shape {
//     Point(Point),
//     Circle(Circle),
// }
//
// pub fn main() {
//     let p = Point { width: 0, height: 7 };
//
//     match p {
//         Point { width, height: 0 } => {
//             println!("{}", width);
//         }
//         Point { width, height } => {
//             println!("{} {}", width, height);
//         }
//     }
//
//     let _shape0 = Shape::Circle(Circle { radious: 10 });
//     let shape = Shape::Point(Point{width: 10, height: 15 });
//
//     let circle = match shape {
//         Shape::Point(Point) => vec![Point { width: 2, height: 2 }],
//         Shape::Circle(Circle) =>  vec![Circle { radious: 4 }]
//
//     };
//     println!("{:?}", circle);
// }

pub fn main() {
   let v1 = vec![1,2,3]; 
   let v2 = vec!['a','b','c','d'];
   let last1 = return_last(v1);
   let last2 = return_last(v2);
   println!("1 = {}, 2= {}", last1, last2);
}

fn return_last<T: Copy>(v: Vec<T>) -> T {
    v[v.len()-1]
}
