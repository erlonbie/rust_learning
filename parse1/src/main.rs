#![allow(deprecated)]
#![allow(dead_code)]
#![allow(unused_variables)]

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseMyLogError {
    #[error("invalid format")]
    FormatInvalid,
    #[error("invalid timestamp")]
    TimestampInvalid,
    #[error("invalid id")]
    InvalidId,
}

fn my_try_from(line: &str) -> std::result::Result<std::result::Result<i32, std::num::ParseIntError>, ParseMyLogError> {
    type Error = ParseMyLogError;

    let mut parts = line.split_whitespace();

    let id = match parts.next().ok_or(ParseMyLogError::FormatInvalid) {
        Ok(it) => it,
        Err(err) => return Err(err),
    }.parse::<i32>();
    Ok(id)
}

fn main() {
    let line = "1 erlon 2020-01-01";
    let log = my_try_from(line).unwrap();

    println!("{log:?}");
    // let line = "this is a string for test";
    // let tokens = line.split_whitespace().collect::<Vec<&str>>();
    // println!("{tokens:?}");
    // for t in tokens {
    //     println!("{}", t.len());
    // }
    //
}
