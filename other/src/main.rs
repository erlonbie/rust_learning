use std::collections::btree_map::{BTreeMap, ValuesMut};
use std::collections::HashMap;

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = ['a', 'b', 'c'];
    let dic: HashMap<_, _> = arr1.iter().zip(arr2.iter()).collect();
    println!("{:?}", dic);
    for (key, value) in dic {
        println!("{:?} : {:?}", key, value);
    }

    let arr3 = vec![4, 5, 6];
    let arr4 = vec!['d', 'e', 'f'];
    let dic2: HashMap<_, _> = arr3.iter().zip(arr4.iter()).collect();
    println!("{:?}", dic2);
    for (key, value) in dic2.iter() {
        println!("{:?} : {:?}", key, value);
    }
    println!("{:?}", dic2);

    let mut aa: BTreeMap<_, _> = BTreeMap::new();
    aa.insert("abc", 123);
    aa.insert("def", 456);

    println!("{:?}", aa);
    for (key, value) in aa.iter() {
        println!("{:?} : {:?}", key, value);
    }

    for i in arr1 {
        println!("{}", i);
    }
}
