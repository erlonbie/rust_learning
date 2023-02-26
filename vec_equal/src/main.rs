#[derive(Debug)]
struct MyStruct {
    name: String,
    body: String,
    published: bool,
}

impl PartialEq for MyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl MyStruct {
    fn new(name: String, body: String, published: bool) -> Self {
        Self {
            name,
            body,
            published,
        }
    }
}

fn main() {
    let v1 = vec![
        MyStruct::new("Matrix".to_string(), "Sample body".to_string(), true),
        MyStruct::new("Matrix2".to_string(), "Sample body".to_string(), true),
        MyStruct::new("Matrix3".to_string(), "Sample body".to_string(), true),
    ];

    let v2 = vec![
        MyStruct::new("Matrix2".to_string(), "Sample body".to_string(), false),
        MyStruct::new("Matrix3".to_string(), "Sample body".to_string(), false),
        MyStruct::new("Matrix4".to_string(), "Sample body".to_string(), false),
    ];

    let mut v3 = Vec::new();
    v3.extend(v1);
    for mut x in v2 {
        if !v3.contains(&x) {
            v3.push(x);
        } else {
            x.body = "both".to_string(); 
            println!("{x:?}");
        }
    }
    // println!("{:?}", v3);
}
