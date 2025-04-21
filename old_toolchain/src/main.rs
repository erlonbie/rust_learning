use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;

trait Displayable: Display + Any {}
impl<T: Display + Any + Deref + PartialEq> Displayable for T {}

struct Node {
    value: Box<dyn Displayable>,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: impl Displayable) {
        let new_node = Box::new(Node {
            value: Box::new(value),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn search(&self, value: &dyn Displayable) -> Option<&Node> {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if format!("{}", node.value) == format!("{}", value) {
                return Some(node);
            }
            current = node.next.as_ref();
        }
        None
    }}

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}
// impl Displayable for Person {}

fn main() {
    let mut my_list = LinkedList::new();
    let my_int = &5;
    let my_string = String::from("Hello");
    let my_person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let my_bool = &true;

    my_list.push(my_int);
    my_list.push(my_bool);
    my_list.push(my_string);
    my_list.push(my_person);

    let result = my_list.search(&String::from("Hello"));
    println!("FOUND IT {:?}", result.is_some());
    let result = my_list.search(&&5);
    println!("FOUND IT {:?}", result.is_some());


    let mut current = my_list.head.as_ref();
    while let Some(node) = current {
        println!("{}", node.value);
        current = node.next.as_ref();
    }
}

