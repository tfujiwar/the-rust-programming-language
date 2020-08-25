fn main() {
    let mut s = String::from("こんにちは");
    s.push_str("、世界");
    println!("{}", s);

    let t = s + "！！";
    println!("{}", t);
    // println!("{}", s);

    println!("{}", t.len());

    for i in t.bytes() {
        println!("{}", i);
    }

    for i in t.chars() {
        println!("{}", i);
    }
}
