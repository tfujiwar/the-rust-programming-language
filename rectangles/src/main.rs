fn main() {
    let r = Rectangle {
            width: 3,
            height: 4,
    };
    println!("{:?}: {}", r, area(&r));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
