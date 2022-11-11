#[derive(PartialEq, Eq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

// fn fit_in_my_feet(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|shoe| shoe.size == size).collect()
// }

// impl Shoe {
//     pub fn fit_in_my_feet(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
//         shoes.into_iter().filter(|shoe| shoe.size == size).collect()
//     }
// }

#[derive(Debug)]
struct Count {
    count: u32,
}

impl Count {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Count {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // let v1 = vec![1,2,3,4,5];
    // let v2: Vec<i32> = v1.iter().map(|x| x*x).collect();
    // println!("{:?}", v2);

        let count  = Count::new();
        println!("{:?}", count);

        for i in count.skip(0) {
                print!("{:?} ", i);
        }
        println!();

        let iter  = Count::new().next().expect("Deu ruim");
        println!("{:?}", iter);

        let zipped_vec: Vec<_> = Count::new()
            .zip(Count::new().skip(1)).collect();
        println!("{:?}", zipped_vec);

        let mapped_vec: Vec<u32> = Count::new()
            .zip(Count::new().skip(1))
            .map(|(a, b)| a * b).collect();
        println!("{:?}", mapped_vec);

        let filtered_vec: Vec<u32> = Count::new()
            .zip(Count::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0).collect();
        println!("{:?}", filtered_vec);

        let sum: u32 = Count::new()
            .zip(Count::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("{:?}", sum);

        let a = [1,2,3];
        let squares: Vec<_> = a.iter().map(|x| x*x).collect();
        println!("{:?}", squares);

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_fit_in_my_feet() {
//         let shoes = vec![
//             Shoe {
//                 size: 10,
//                 style: String::from("sneaker"),
//             },
//             Shoe {
//                 size: 13,
//                 style: String::from("sandal"),
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot"),
//             },
//         ];
//
//         let in_my_size = fit_in_my_feet(shoes, 10);
//
//         assert_eq!(
//             in_my_size,
//             vec![
//                 Shoe {
//                     size: 10,
//                     style: String::from("sneaker"),
//                 },
//                 Shoe {
//                     size: 10,
//                     style: String::from("boot"),
//                 },
//             ]
//         )
//     }
//
//     #[test]
//     fn test_count_iterator() {
//         let mut count = Count::new();
//         assert_eq!(count.next(), Some(1));
//         assert_eq!(count.next(), Some(2));
//         assert_eq!(count.next(), Some(3));
//         assert_eq!(count.next(), Some(4));
//         assert_eq!(count.next(), Some(5));
//         assert_eq!(count.next(), None);
//     }
//     #[test]
//     fn test_other_iterator_trait_method() {
//         let sum: u32 = Count::new()
//             .zip(Count::new().skip(1))
//             .map(|(a, b)| a * b)
//             .filter(|x| x % 3 == 0)
//             .sum();
//         assert_eq!(18, sum);
//     }
// }
