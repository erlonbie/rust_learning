use std::{fmt::Display, ops::{Add, Sub}};

#[derive(Clone, Copy, PartialEq)]
struct MyNumber {
    n1: i32,
    n2: i32,
}

impl MyNumber {
    fn new(n1: i32, n2: i32) -> Self {
        println!("criei uma cópia");
        Self { n1, n2 }
    }
}

impl Display for MyNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.n1, self.n2)
    }
}

impl Add for MyNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n1: self.n1 + rhs.n1,
            n2: self.n2 + rhs.n2,
        }
    }
}

impl Sub for MyNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            n1: self.n1 - rhs.n1,
            n2: self.n2 - rhs.n2,
        }
    }
}

fn something(n: &MyNumber) {
    println!("{}", n);
}

fn main() {
    let m1 = MyNumber::new(1, 2);

    something(&m1);
    let m2 = MyNumber::new(1, 2) - m1;
    something(&m2);
    let m3 = m1 + m1;
    something(&m3);
    let m4 = m2 + m3;
    something(&m4);
}
