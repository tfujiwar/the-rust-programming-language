fn main() {
    let ipv4 = IPAddr::V4(127, 0, 0, 1);
    let ipv6 = IPAddr::V6(String::from("::1"));
    ipv4.print();
    ipv6.print();
}

#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPAddr {
    fn print(&self) {
        println!("{:?}", self);
    }
}
