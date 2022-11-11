// use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    // let v = vec![1, 2, 3, 4, 5];
    // let v2 = vec!["apple", "banana", "pear"];

    // let third = &v[2];
    // v.push(6);
    // // println!("The third element is {}", third);
    //
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element")
    // }

    // for i in &v {
    //     let j = i + 50;
    //     println!("{}", j);
    // }
    //
    // for i in v {
    //     println!("{}", i);
    // }
    //
    // for f in &v2 {
    //     let mut f1 = f.to_string();
    //     f1.push('😄');
    //     println!("{:?}", f1);
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String)
    // }
    //
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.22)
    // ];
    //
    // match &row[1] {
    //   SpreadsheetCell::Int(i) => println!("{}",i),
    //   _ => println!("Not a integer!")
    // }

    // let s1 = String::from("foo");
    // let s2 = String::from("bar");
    // let s3 = "hello";
    //
    // let s4 = format!("{}{}", s1, s2);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);
    // println!("{}", s4);
    //
    // let jp = String::from("ありがとうございました");
    //
    // for j in jp.bytes() {
    //     println!("{}", j);
    // }
    //
    // for j in jp.chars() {
    //     println!("{}", j);
    // }
    //
    // for j in jp.graphemes(true) {
    //     println!("{}", j);
    // }

    let mut scores = HashMap::new();
    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");
    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);
    //
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("{:?}", score.unwrap());
    //
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 20);
        scores.entry(String::from("Yellow")).or_insert(30);
        scores.entry(String::from("Yellow")).or_insert(40);
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
        map.entry(word).and_modify(|counter| *counter +=1).or_insert(1);
    }
    println!("{:?}", map);

    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    println!("{:?}", letters);
    // assert_eq!(letters[&'s'], 2);
    // assert_eq!(letters[&'t'], 3);
    // assert_eq!(letters[&'u'], 1);
    // assert_eq!(letters.get(&'y'), None);
}
