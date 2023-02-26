use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_intensity, simulated_random_number);
    generate_workout(simulated_intensity + 1, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Cacher2<T>
where
    // T: Fn(u32) -> u32,
    T: Fn(u32) -> u32 + std::hash::Hash + std::cmp::Eq,
    // T: Copy,
{
    calculation: T,
    meu_hash: Option<HashMap<T, T>>,
}

impl<T> Cacher2<T>
where
    // T: Fn(u32) -> u32,
    T: Fn(u32) -> u32 + std::hash::Hash + std::cmp::Eq,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            meu_hash: Some(HashMap::new()),
        }
    }

    fn value(&mut self, arg: T) -> T {
        match self.meu_hash {
            Some(m) => match m.get(&arg) {
                Some(_v) => todo!(),
                None => {
                    let v = (self.calculation)(arg);
                    self.meu_hash.expect("sadf").insert(arg, v);
                    v
                }
            },
            None => {
                let v = (self.calculation)(arg);
                self.meu_hash = Some(HashMap::new());
                self.meu_hash.as_mut().expect("ASHAKSJH").insert(arg, v);
                v
            }
        }
    }
}

// struct Cacher2<T>
// where
//     T: Fn(T) -> T + std::hash::Hash + std::cmp::Eq,
//     T: Copy,
// {
//     calculation: T,
//     meu_hash: Option<HashMap<T, T>>,
// }
//
// impl<T> Cacher2<T>
// where
//     T: Fn(T) -> T + std::hash::Hash + std::cmp::Eq,
//     T: Copy,
// {
//     fn new(calculation: T) -> Self {
//         Self {
//             calculation,
//             meu_hash: None,
//         }
//     }
// }

// impl<T> Cacher2<T> {
//     fn new(calculation: T) -> Self {
//         Self {
//             calculation,
//             meu_hash: None,
//         }
//     }
// }

// impl<T> Cacher2<T>
// where
//     T: Fn(T) -> T + std::hash::Hash + std::cmp::Eq,
//     T: Copy,
// {
//     // fn new(calculation: T) -> Self {
//     //     Self {
//     //         calculation,
//     //         meu_hash: None,
//     //     }
//     // }
//
//     fn value(&mut self, arg: T) -> T {
//         match &self.meu_hash {
//             Some(m) => match m.get(&arg) {
//                 Some(_v) => todo!(),
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.meu_hash.as_mut().expect("ASHAKSJH22").insert(arg, v);
//                     v
//                 }
//             },
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.meu_hash = Some(HashMap::new());
//                 self.meu_hash.as_mut().expect("ASHAKSJH").insert(arg, v);
//                 v
//             }
//         }
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num * 2
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Today, do {} situps!", cached_result.value(intensity * 2));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes", cached_result.value(intensity));
    }
}
