fn main() {
    let x = 1;
    if x % 2 == 0 {
        println!("even");
    } else {
        println!("odd")
    }

    let y = if x % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("{}", y);

    let mut i = 3;
    loop {
        println!("{}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }

    i = 3;
    while i != 0 {
        println!("{}", i);
        i = i - 1;
    }

    let arr = [1, 2, 3];
    for elem in arr.iter() {
        println!("{}", elem);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
}
