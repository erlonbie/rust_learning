struct ImportantText<'a> {
    part: &'a str,
}

impl<'a> ImportantText<'a> {
    /// Makes a big announcemnt
    fn return_part(&self, announcemnt: &str) -> &str {
        println!("Attenntion please: {}", announcemnt);
        self.part
    }
}

use std::fmt::Display;

/// Find the biggest string and makes a big announcemnt
fn longest_with_an_annauncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Attenntion!: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abc");
    let string2 = String::from("defg");
    let result = longest(string1.as_str(), string2.as_str());
    println!("the longest string is {}", result);

    let string3 = String::from("Isso é um teste. Isso é outro teste");
    let string4 = string3.split('.').next().expect("Não funcionou");
    let i = ImportantText { part: string4 };
    println!("{}", i.part);
    i.return_part("HELLO");

    longest_with_an_annauncement(&string1, &string2, "HELLO2");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if ({
        let this = &x;
        this.as_bytes().len()
    }) > y.len()
    {
        x
    } else if y.len() > x.len() {
        y
    } else {
        "none"
    }
}
