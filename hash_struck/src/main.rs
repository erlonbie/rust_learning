use std::collections::HashMap;

struct MyStr {
    v: i32,
    m:  Option<HashMap<i32, i32>>,
}

impl MyStr {
    /// Creates a new [`MyStr`].
    fn new(v: i32) -> Self {
        Self { v, m: Some(HashMap::new()) }
    }
}

fn main() {

    let mut a = MyStr::new(4);
    let v = a.v;

    a.m.as_mut().unwrap().insert(1, 2);
    a.m.as_mut().unwrap().insert(2, 4);
    a.m.as_mut().unwrap().insert(3, 6);


    println!("{} {}", a.v, v);

    for (k, v) in a.m.as_mut().unwrap().iter() {
        println!("{} - {}", k, v);

    }

    // a.m.expect("some value").insert(2, 4);

    // a.m.expect("abc").insert(1, 1);
    // MyStr::new(4).m.expect("abc").insert(2, 4);
    // MyStr::new(4).m.expect("abc").insert(3, 6);
}
