#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

impl Library {
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.books.pop().map(|book| book + " is found")
    }
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("Book 1");
    my_library.add_book("Book 2");
    my_library.add_book("Book 3");
    my_library.add_book("Book 4");

    for book in my_library {
        println!("{book}");
    }

    let v1 = vec![1, 2, 3];
    let v2 = v1.iter().enumerate().map(|(i, v)| (i,v*v)).collect::<Vec<(usize, i32)>>();

    println!("{v2:?}");
}
