#![allow(unused)]
// fn print_int(x: u32) {
//     println!("{x}");
// }
//
// enum Option<T> {
//     None,
//     Some(T)
// }

use std::fs::File;

fn main() {
    let arr: [u32; 3] = [1u32, 2, 3];
    let first_element = arr.get(5);
    // print_int(first_element);
    let len = "some text".len();
    println!("{:?}, {len}", first_element);
    println!("{:?}", arr);

    let v = vec![0,1,2,3];
    println!("{:?}", v.get(6));

    let fruits = vec!["🍌", "🍎", "🥥"];
    let first = fruits.first();
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // let f = File::open("hello.tx");
    // let f = match f {
    //    Ok(file)  => file,
    //    Err(error) => panic!("Can't open the file {:?}", error)
    // };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

}
