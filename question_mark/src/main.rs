use std::{collections::HashMap, num::ParseIntError};

// fn parse_str(input: &str) -> Result<i32, ParseIntError> {
//     let parsed_number = input.parse::<i32>()?;
//     Ok(parsed_number)
//
// }

fn parse_str(input: &str) -> Result<u8, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<u8>()?;
    Ok(parsed_number)
}

fn main() {
    let str_vec = vec!["seven", "8", "9.0", "Nic", "5060"];
    let mut my_hash = HashMap::new();

    for (i, item) in str_vec.iter().enumerate() {
        let parsed = parse_str(item);
        // println!("{:?}", parsed);
        //
        // if let Ok(var) = parsed {
        //     println!("{}", var);
        //
        // }

        match parsed {
            Ok(number) => {
                my_hash.insert(i, number);
                println!("{}", number)
            }
            Err(e) => println!("Error {:?}", e),
        }

        // if parsed.is_ok() {
        //     println!("{:?}", parsed.unwrap());
        // }
    }

    for (k, v) in my_hash {
        println!("{}: {}", k, v);
    }
}
