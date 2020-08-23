fn main() {
    let x = String::from("abcde");
    let z;
    let c1;
    let c2;
    {
        let y = String::from("abcd");
        c1 = conststr(x.as_str(), y.as_str());
        c2 = conststr(y.as_str(), x.as_str());
        z = longest(x.as_str(), y.as_str());

        println!("{}", c1);
        println!("{}", c2);
        println!("{}", z);
    }

    println!("{}", c1);
    // println!("{}", c2);
    // println!("{}", z);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn conststr<'a, 'b>(_x: &'a str, _y: &'b str) -> &'a str {
    "const"
}
