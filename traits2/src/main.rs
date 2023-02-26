pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         self.author.to_string()
//     }
//     fn summarize(&self) -> String {
//         format!("{}, by {}", self.headline, self.author)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         self.username.to_string()
//     }
//
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub fn notity_sugar(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notity_no_sugar<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    /// Creates a new [`Pair<T>`].
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    /// Returns the cmp displays of this [`Pair<T>`].
    fn cmp_displays(&self) {
        if self.x >= self.y {
            println!("The largest member is = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Tweet username: {}", tweet.summarize_author());
    notity_sugar(&tweet);
    notity_no_sugar(&tweet);
    println!("Article summary: {}", article.summarize());
    println!("Article author: {}", article.summarize_author());
    notity_sugar(&article);
    notity_no_sugar(&article);
}
