use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("a", 10);
    scores.insert("b", 20);

    println!("{:?}", scores);
    println!("{:?}", scores.get("a"));

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    println!("{:?}", scores.entry("b"));
    println!("{:?}", scores.entry("c"));

    println!("{:?}", scores.entry("b").or_insert(100));
    println!("{:?}", scores.entry("c").or_insert(100));
}
