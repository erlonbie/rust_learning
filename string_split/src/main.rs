fn main() {
    let s1 = String::from("export abc 123");
    // let s2 = String::from("abc export 123");
    // let s3 = String::from("abc 123 export");
    let split = s1.split(' ');
    let vec = &split.collect::<Vec<&str>>();
    for s in vec {
        println!("{}", s);
    }
}
