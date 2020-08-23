fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{:?}: {}", p, p.l2_norm());
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn l2_norm(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}
