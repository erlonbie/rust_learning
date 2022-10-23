use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // a();

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(ec) => panic!("Problem creating the file {:?}", ec)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };

    // let f = File::open("hello2.txt").expect("Failed to open hello2.txt");

    read_user_from_file1();
    read_user_from_file2();
    read_user_from_file3();
    read_user_from_file4();


}

fn read_user_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_user_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn a() {
//     b();
// }
//
// fn b() {
//     c(22);
// }
//
// fn c(num: i32) {
//     if num == 22 {
//         panic!("Don't pass in 22!");
//     }
// }
