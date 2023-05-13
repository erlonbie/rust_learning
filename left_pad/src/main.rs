fn left_pad(input: &str, pad: &str, length: usize) -> String {
    let mut result = String::new();
    let mut pad_length = length - input.len();
    while pad_length > 0 {
        result.push_str(pad);
        pad_length -= pad.len();
    }
    result.push_str(input);
    result
}

fn main() {
    let result = left_pad("hello", "_", 7);
    println!("{}", result);
}
