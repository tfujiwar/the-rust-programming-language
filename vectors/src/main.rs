fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("{:?}", v1);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    for i in 0..5 {
        if let Some(num) = v2.get(i) {
            println!("{}", num);
        } else {
            println!("does not exist");
        }
    }

    for i in &mut v1 {
        *i += 10;
    }
    println!("{:?}", v1);
}
