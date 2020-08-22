fn main() {
    func(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x = {}", x);
    println!("y = {}", y);

    println!("x + y = {}", add_sub(x, y).0);
    println!("x - y = {}", add_sub(x, y).1);
}

fn func(x: i32) {
    println!("x = {}", x);
}

fn add_sub(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y)
}
