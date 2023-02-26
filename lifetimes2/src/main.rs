// fn printa_array<T>(input: &[T])
// where
//     T: std::fmt::Display,
// {
//     for x in input {
//         print!("{x} ");
//     }
//     println!();
// }
//
// pub fn main() {
//     let v1 = [1,2,3];
//     let v2 = vec![1,2,3];
//     let v3 = vec!["abc","def","ghi"];
//     let v4 = vec![String::from("jkl"),String::from("mno"),String::from("pqr")];
//     printa_array(&v1);
//     printa_array(&v2);
//     printa_array(&v3);
//     printa_array(&v4);
// }

// use std::fmt::{Display, Formatter, Result};
//
// extern crate colour;
//
// #[derive(Debug)]
// struct City<'a> {
//     name: &'a str,
//     date_founded: u32,
// }
//
// struct Country {
//     name: String,
//     date_founded: u32,
// }
//
// impl<'a> Display for City<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(
//             f,
//             "{{ name: {}, date_found: {} }}",
//             self.name, self.date_founded
//         )
//     }
// }
//
// impl<'a> City<'a> {
//     fn new(name: &'a str, date_founded: u32) -> Self {
//         Self { name, date_founded }
//     }
// }
//
// fn print_citty<'a, T>(input: &'a T)
// // where
// //     T: Display,
// {
//     // println!("{input}");
//     println!("{}", input.name);
//     println!("{}", input.date_founded);
// }
//
// fn main() {
//     let c1 = City {
//         name: &String::from("Kyoto"),
//         date_founded: 1888,
//     };
//
//     let c2 = City::new("Tokyo", 1800);
//
//     // println!("{c1}");
//     colour::red_ln!("{}", c1);
//     println!("{}", c1.name);
//     println!("{}", c1.date_founded);
//
//     print_citty(&c2);
// }

use std::fmt::Display;

#[derive(PartialOrd)]
struct Pessoa<'a> {
    primeiro_nome: &'a str,
    segundo_nome: &'a str,
}

impl<'a> Display for Pessoa<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.primeiro_nome, self.segundo_nome)
    }
}

impl<'a> PartialEq for Pessoa<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.primeiro_nome == other.primeiro_nome && self.segundo_nome == other.segundo_nome
    }
}

impl<'a> Pessoa<'a> {
    fn new(primeiro_nome: &'a str, segundo_nome: &'a str) -> Self {
        Self {
            primeiro_nome,
            segundo_nome,
        }
    }
}

/// Returns a referencen to T acording to op operation of type O which implements Fn(&T, &T) -> bool
fn retorna_operacao<'a, T, O>(input1: &'a T, input2: &'a T, op: O) -> &'a T
where
    T: std::cmp::PartialOrd,
    O: Fn(&T, &T) -> bool
{

   if op(input1, input2) {
       input1
   }
   else {
       input2
   }
    // if input1 < input2 {
    //     input1
    // } else {
    //     input2
    // }
}

struct Vertex3 {
    x: i32,
    y: i32,
    z: i32
}

impl PartialEq for  Vertex3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Vertex3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.z.partial_cmp(&other.z)
    }
}

impl Display for Vertex3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Vertex3 {
    fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
}


pub fn main() {
    let p2 = Pessoa::new("Erlon", "Bié");
    let p1 = Pessoa::new("Marília", "Marinho");

    let a = retorna_operacao(&9, &11, |a, b| a < b);
    println!("{a}");

    let b = retorna_operacao(&p1, &p2, |a, b| a < b);
    println!("{b}");

    let c = retorna_operacao(&p1, &p2, |a, b| a > b);
    println!("{c}");

    let v1 = Vertex3::new(5,2,3);
    let v2 = Vertex3::new(4,5,6);

    let d = retorna_operacao(&v1, &v2, |a, b| a < b);
    println!("{d}");

}
