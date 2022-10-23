#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn string_value(&self) -> String {
        match &*self {
            IpAddrKind::V4(a,b,c,d) => format!("{}.{}.{}.{}",a,b,c,d),
            IpAddrKind::V6(a) => a.to_owned(),
        }

    }
}

struct IpAddr<'a> {
    kind: &'a IpAddrKind,
    address: String,
}

fn main() {

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let ipaddr1 = IpAddr {
        kind: &localhost,
        address: localhost.string_value(),
    };
    println!("kind: {:?}", ipaddr1.kind);
    println!("kind: {:?}", ipaddr1.address);

    let localhost2 = IpAddrKind::V6(String::from("200.233.AXB.ABC"));
    let ipaddr2 = IpAddr {
        kind: &localhost2,
        address: localhost2.string_value()
    };
    println!("kind: {:?}", ipaddr2.kind);
    println!("kind: {:?}", ipaddr2.address);
}
