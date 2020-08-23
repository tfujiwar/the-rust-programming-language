fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}, {}", s, length(&s));
}

fn length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
