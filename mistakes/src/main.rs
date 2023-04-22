use itertools::Itertools;

#[derive(Debug, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    let points = vec![1, 2, 3, 4];
    points.iter().for_each(|x| println!("{x}"));
    // let diff: Vec<_> = points.windows(2).map(|slice| slice[1] - slice[0]).collect();
    // let diff2: Vec<_> = points
    //     .windows(2)
    //     .map(|s: &[i32]| s.chunks(2).next().unwrap())
    //     .collect();
    //
    // println!("{diff:?}");
    // println!("{diff2:?}");
    // println!("{points:?}");
    //
    // let arr0 = [1, 2, 3, 4];
    // let powerset0 = any_powerset(&arr0);
    // println!("{powerset0:?}");
    //
    // let arr1 = vec![1, 2, 3];
    // let powerset1 = any_powerset(&arr1);
    // println!("{powerset1:?}");
    //
    // let arr2 = vec!["🍌", "🍎", "🍑"];
    // let powerset2 = any_powerset(&arr2);
    // println!("{powerset2:?}");
    //
    // let arr3 = vec![true, false];
    // let powerset3 = any_powerset(&arr3);
    // println!("{powerset3:?}");
    //
    // let p1 = Point::default();
    // println!("{}", p1.x);
    // println!("{}", p1.y);
    // let p2 = Point { x: 1, y: 1 };
    // let arr4 = vec![p1, p2];
    // let powerset4 = any_powerset(&arr4);
    // println!("{powerset4:?}");
}

fn any_powerset<T>(input: &[T]) -> Vec<Vec<&T>> {
    (0..=input.len())
        .flat_map(|n| input.iter().combinations(n))
        .collect::<Vec<_>>()
}
