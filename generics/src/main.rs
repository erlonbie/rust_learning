#[derive(Debug)]
struct Point<T, U>
where
    T: Copy,
    U: Copy,
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    fn mixup<V, W>(&self, other: &Point<V, W>) -> Point<T, W>
    where
        V: Copy,
        W: Copy,
    {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("largest: {}", largest);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest(number_list2);
    println!("largest: {}", largest);

    let number_list3 = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(number_list3);
    println!("largest: {}", largest);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.2, y: 10.25 };
    let p3 = Point { x: 5, y: 10.25 };
    println!("p1.x: {:?}, p2.y: {:?}, p3: {:?}", p1.x, p2.y, p3);
    println!("p1.x: {:?}, p2.y: {:?}, p3: {:?}", p1.x(), p2.y(), p3);

    let p4 = Point { x: 5, y: 10.4 };
    let p5 = Point { x: "Hello", y: 'c' };

    let p6 = &p4.mixup(&p5);
    println!("p6.x: {}, p6.y: {}", p6.x(), p6.y());

    let p7 = &p5.mixup(&p4);
    println!("p7.x: {}, p7.y: {}", p7.x(), p7.y());
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
