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
}
