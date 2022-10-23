#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // println!("what is your name?");
    // let mut name: string = string::new();
    // let greeting = "nice to meet you";
    // io::stdin().read_line( &mut name)
    //     .expect("didn't receive input");
    //
    // println!("hello {}!, {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.1415592;
    // let age = "42";
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    // age += 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize: {}", usize::MAX);
    // println!("Max u128: {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f32 : {}", f32::MAX);

    // let boolean_variable: bool = true;
    // let my_grade = 'A';

    // let num_1: f32 = 1.111111111111111;
    // println!("f32 : {}",  num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64 : {}", num_2 + 0.111111111111111);

    // let num_3: u32 = 5;
    // let num_4: u32 = 4;
    // println!("5 + 4 = {}", num_3 + num_4);
    // println!("5 - 4 = {}", num_3 - num_4);
    // println!("5 * 4 = {}", num_3 * num_4);
    // println!("5 / 4 = {}", num_3 / num_4);
    // println!("5 % 4 = {}", num_3 % num_4);

    // let randof_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("Random : {}", random_num);

    // let age: i32 = 65;
    // if (1..18).contains(&age) {
    //     println!("Important birthday");
    // }
    // else if (19..51).contains(&age) {
    //    println!("Some important");
    // }
    // else {
    //     println!("Not important")
    // }

    // let mut my_age = 47;
    // let can_vote = my_age >= 18;
    // println!("Can Vote : {}", can_vote);

    // let mut my_age = 47;
    // let age2 = 18;
    // match age2 {
    //     1..=18 => println!("Important Birthday 1"),  // 1 through 18
    //     21 | 50 => println!("Important Birthday 2"), // 21 or 50
    //     65..=i32::MAX => println!("Important Birthday 3"), // > 65
    //     _ => println!("Not an Important Birthday"), // Default
    // };
    //
    // my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Can't Vote"),
    //     Ordering::Greater => println!("Can Vote"),
    //     Ordering::Equal => println!("You just gained the right to vote!"),
    // };

    // let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut loop_idx = 0;
    // loop {
    //     if arr_2[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //
    //     if arr_2[loop_idx] == 9 {
    //         break;
    //     }
    //
    //     println!("Val : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    //
    // loop_idx = 0;
    // while loop_idx < arr_2.len() {
    //     println!("Arr : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    //
    // for val in arr_2.iter() {
    //     println!("Val : {}", val);
    // }

    // let int1: i32 = 123;
    // let int2 = Box::new(2);
    // let st1: &str = "string test";
    // let st2: String = String::new();
    // let st3: String = String::from("123");
    // let st4: String = st1.to_string();
    // let byte_arr = st4.as_bytes();
    // let st5 = &st4[0..6];
    // print_type_of(&int1);
    // print_type_of(&int2);
    // print_type_of(&st1);
    // print_type_of(&st2);
    // print_type_of(&st3);
    // print_type_of(&st4);
    // print_type_of(&st5);
    // print_type_of(&byte_arr);
    // println!("{}", byte_arr.len());
    // println!("{}", st5.len());
    //
    // for i in byte_arr {
    //     println!("{}", i);
    // }
    //
    // for i in st3.bytes() {
    //     println!("{}", i);
    // }

    // enum Day {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // }
    //
    // // impl Day {
    // //     fn is_weekend(&self) -> bool {
    // //         match self {
    // //             Day::Saturday | Day::Sunday => true,
    // //             _ => false,
    // //         }
    // //     }
    // // }
    //
    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         matches!(self, Day::Monday | Day::Friday)
    //     }
    // }
    //
    // let today: Day = Day::Saturday;
    //
    // match today {
    //     Day::Monday => println!("Everyone hates Monday"),
    //     _ => println!("whatever")
    //     // Day::Tuesday => println!("Donut day"),
    //     // Day::Wednesday => println!("Hump day"),
    //     // Day::Thursday => println!("Pay day"),
    //     // Day::Friday => println!("Almost Weekend"),
    //     // Day::Saturday => println!("Weekend!!!"),
    //     // Day::Sunday => println!("Weekend!!!"),
    // }
    //
    // println!("Is today the weekend {}", today.is_weekend());

    // let _vec1: Vec<i32> = Vec::new();
    //
    // let mut vec2 = vec![1, 2, 3, 4];
    //
    // vec2.push(5);
    //
    // println!("1st : {}", vec2[5]);
    // let _second: &i32 = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd : {}", second),
    //     None => println!("No 2nd value"),
    // };
    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    //
    // for i in &vec2 {
    //     println!("{}", i);
    // }
    //
    // println!("Vec Length : {}", vec2.len());
    //
    // println!("Pop {:?}", vec2.pop());

    // let mut my_vec: Vec<i32> = vec![1,2,3];
    // println!("{:?}", my_vec);
    // add_to_vector(&mut my_vec);
    // println!("{:?}", my_vec);

    // let say = String::from("cat");
    // let say2 = &say;
    // println!("{}", say);
    // println!("{}", say2);
    // let say = 123;
    // println!("{}", say);

    let a: i32 = 5;
    let b: i32 = 22;
    let x = 2.25;
    let y: f32 = 1.75;
    let sum = a + b;
    let difference = x - 1.0;
    let product = 4 * a;
    let quotient = 9.0 / y;
    let remainder = a % b;
    let t = true;
    let c = '🦀';
    let arr1 = [1, 2, 3, 4];
    let fourth = arr1[3];
    let text_len = "some text".len();
    let tupla = ('🥹', 123, "texto", true);
    println!(
        "{} {} {} {} {} {} {} {} {}",
        a, b, x, y, t, c, fourth, text_len, tupla.0
    );
    let st1_1 = Struct1 {
        age: 29,
        name: String::from("Erlon"),
        activated: true,
    };

    let mut st1_2 = Struct1 {
        age: 30,
        name: String::from("Erlon"),
        activated: true,
    };

    let Struct1 {
        mut age,
        name,
        activated,
    } = &st1_1;
    age += 2;
    println!("{:?} {:?} {:?}", st1_1.age, st1_2.age, age);
    let we1 = WebEvent::KeyPress('q');

    let something = Some(1);
    let val1 = get_sum(a, b);
    println!("val1 : {}", val1);
    println!("a : {}, b: {}", a, b);



}

fn get_sum(mut x:i32, mut y: i32) -> i32 {
    x += 1;
    y += 1;
    x + y
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

enum Option<T> {
    Some(T),
    None
}

struct Struct1 {
    name: String,
    age: i32,
    activated: bool,
}

struct Struct2(String, i32, bool);

// fn add_to_vector (a_vec: &mut Vec<i32>) {
//     a_vec.push(4);
// }
//
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
