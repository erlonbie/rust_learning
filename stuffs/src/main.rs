fn main() {

    // let my_string = String::from("abcde");
    // let my_string_view = (my_string + "z").as_str();
    // println!("{:?}", my_string_view);

    // let mut my_vector = Vec::new();
    // {
    //     let my_string = String::from("abcde");
    //     my_vector.push(&my_string);
    // }

    // let b = my_vector.get(0).expect("deu ruim3").chars().next().expect("deu ruim");
    // println!("{:?}", my_vector.get(0).expect("deu ruim aqui").chars().nth(1).expect("deu ruim 2"));
    // println!("{:?}",b);

    // let mut my_vector = Vec::new();
    // {
    //     let my_string = String::from("abcde");
    //     my_push_back(&mut my_vector, &my_string);
    // }
    // println!("{:?}",my_vector);

    let mut char_array = ['a','b'];
    let (first_slice,second_slice) = char_array.split_at_mut(1);
    let first_element = &mut first_slice[0];
    let second_element = &second_slice[0];
    *first_element = *second_element;

    println!("first: {:?}, second: {:?}", first_element, second_element);
    println!("{:?}", char_array);


}

// fn my_push_back<'a>(v: &mut Vec<&'a str>, s: &'a str) {
//     v.push(s);
// }
