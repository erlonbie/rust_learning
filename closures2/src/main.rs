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

    let my_closure = |x| println!("this is a my_closure - {}", x);
    my_closure("Erlon");

    let v1 = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v1.iter().for_each(|x| v2.push(x + 1));
    println!("{v1:?}");
    println!("{v2:?}");
    let v3 = v2.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("{v2:?}");
    println!("{v3:?}");

    let line = "1 erlon 2020-01-01".to_string();
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let tokens = tokens.iter().map(|w| w.to_string()).collect::<Vec<String>>();
    println!("{tokens:?}");
    let mut enumer = tokens.iter().enumerate().collect::<Vec<(usize, &String)>>();
    println!("{enumer:?}");
    
    let binding = "Liloca".to_string();
    enumer[1].1 = &binding;
    println!("{tokens:?}");
    println!("{enumer:?}");

}
