#[derive(Debug)]
struct User {
    username: String,
    email: String,
    singn_in_cout: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("bogdam@gmail.com"),
        username: String::from("bogdam123"),
        active: true,
        singn_in_cout: 1,
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(String::from("kyle@email.com"), String::from("kyle123"));
    let user3 = User {
        email: String::from("james@email.com"),
        username: String::from("james123"),
        ..user2
    };

    println!("{:#?}", user2);



    let rec1 = Rectangle {width: 30, height: 50};
    let rec2 = Rectangle {width: 20, height: 40};
    let rec3 = Rectangle {width: 40, height: 50};
    let square = Rectangle::square(25);

    println!("rect1 can hold rect2: {}", rec1.can_hold(&rec2));
    println!("rect1 can hold rect3: {}", rec1.can_hold(&rec3));

    println!("sauare: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        singn_in_cout: 1,
    }
}
