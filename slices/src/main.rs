fn main() {
    let mut s = String::from("hello world");
    {
        let word = first_word(&s[..]);
        println!("{}", word);
    }
    s.clear();

    let mut arr = [1, 2, 3, 4, 5];
    {
        let subarr = &arr[..2];
        println!("{}, {}", arr.len(), subarr.len());
    }
    arr[4] = 99;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    return &s[..]
}
