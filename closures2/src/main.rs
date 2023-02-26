fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let binding = my_vec.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    let a: Vec<_> = binding.iter().filter(|x| x.1 % 2 == 0).collect();
    let b: Vec<(usize, bool)> = a.iter().map(|x| (x.0, true)).collect();
    let c: Vec<_> = b.iter().map(|x| x.0).collect();

    println!("{my_vec:?}");
    println!("{binding:?}");
    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    let my_closure = || println!("this is a my_closure");
    my_closure();

    let v1 = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v1.iter().for_each(|x| v2.push(x + 1));
    println!("{v1:?}");
    println!("{v2:?}");
    let v3 = v2.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("{v3:?}");
    println!("{v2:?}");
}
