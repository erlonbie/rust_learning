use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn some_func(word: &mut String) {
    *word = word.chars().map(|c| match c {
        'o' => 'e',
        'l' => 'c',
        _ => c,
    }).collect();

    println!("{word}");
}

fn modify_char_at_index(s: &mut String, index: usize, new_char: char) {
    let mut chars: Vec<char> = s.chars().collect();
    if index < chars.len() {
        chars[index] = new_char;
        *s = chars.into_iter().collect::<String>();
    }
}

fn main() {
    let x = MyBox::new(2);
    assert_eq!(*x, 2);

    let mut w = MyBox::new(String::from("Hello"));
    some_func(&mut w);

    let mut s = String::from("hello");
    modify_char_at_index(&mut s, 1, 'a');
    println!("{}", s); // prints "hallo"
}
