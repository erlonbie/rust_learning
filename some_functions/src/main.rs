use std::num::ParseIntError;

fn sum_str(input: &Vec<String>) -> Result<u32, ParseIntError> {
    let mut sum = 0u32;
    // input.iter().for_each(|x| sum += x.parse::<u32>()?);
    for s in input {
        sum += match s.parse::<u32>() {
            Ok(it) => it,
            Err(_err) => 0
        };
    }
    Ok(sum)
    // println!("{sum}");
}

fn main() {
    // let v = vec!["1".to_string(), "2".to_string()];
    let v = vec!["1".to_string(), "5".to_string(), "abc".to_string()];
    let result = sum_str(&v);
    println!("{:?}", result.unwrap());
}
