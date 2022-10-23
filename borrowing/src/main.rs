fn main() {
    // let mut s1 = gives_ownershio();
    // let another_string = "world".to_owned();
    // let s2 = "Outra string";
    // s1.push_str(&another_string);
    // let s3 = format!("{} - {}", s2, s2);
    // let s4 = [s2,s2].join(" | ");
    //
    // // add_a_character(s2);
    //
    // println!("{:?}", s1);
    // println!("{:?}", s2);
    // println!("{:?}", s3);
    // println!("{:?}", s4);
    let mut owned_string: String = "hello".to_owned();
    let borrowed_string: &str = "world";
    let owned_string_ptr = &owned_string;
    println!("{:p}", &owned_string_ptr);

    // let new_owned_string = owned_string + borrowed_string;
    let new_owned_string = format!("{} {}", owned_string, borrowed_string);
    let len = calculate_length(&owned_string);
    let len2 = calculate_length(borrowed_string);

    add_a_character(&mut owned_string);
    let owned_string_ptr = &owned_string;

    println!("{}", new_owned_string);
    println!("{}", owned_string);
    println!("{:p}", owned_string_ptr);
    println!("{}", borrowed_string);
    println!("{}", len);
    println!("{}", len2);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn add_a_character(str: &mut String) {
    str.push('a');
    // str.to_owned().push('a');
    // println!("new str => {:?}", str);
}
//
// fn gives_ownershio() -> String {
//     String::from("Nova String")
// }
