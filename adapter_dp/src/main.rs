trait Target {
    fn request(&self);
}

struct Adaptee;

impl Adaptee {
    fn specific_request(&self) {
        println!("Specific request made.");
    }
}

struct Adapter {
    adaptee: Adaptee,
}

impl Target for Adapter {
    fn request(&self) {
        self.adaptee.specific_request();
    }
}

fn main() {
    let adaptee = Adaptee;
    let adapter = Adapter { adaptee };

    adapter.request();
}

