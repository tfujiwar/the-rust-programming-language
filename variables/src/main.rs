const A: u32 = 100_000;

fn main() {
    println!("{}", A);

    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    let y = 5;
    let y = y + 1;
    println!("y = {}", y);

    let spaces = " ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    let _hoge: i32 = "-123".parse().expect("not a number");

    let byte: u8 = b'A';
    let ch: char = 'A';
    println!("{}, {}", byte, ch);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b, c);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3];
    println!("{}, {}, {}", arr[0], arr[1], arr[2]);
}
