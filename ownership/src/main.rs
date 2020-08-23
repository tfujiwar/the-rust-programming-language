fn main() {
    let mut s = String::from("hello");
    println!("{}, {}, {}", s, s.len(), s.capacity());

    s.push_str(", world");
    println!("{}, {}, {}", s, s.len(), s.capacity());

    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    let t1 = s;
    let t2 = t1.clone();
    println!("{}, {}", t1, t2);
}
