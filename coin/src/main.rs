fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Dime;
    let c4 = Coin::Quarter(UsState::Alaska);
    println!("{:?}: {}", c1, cents(&c1));
    println!("{:?}: {}", c2, cents(&c2));
    println!("{:?}: {}", c3, cents(&c3));
    println!("{:?}: {}", c4, cents(&c4));

    if let Coin::Quarter(state) = c3 {
        println!("Quarter {:?}!!", state);
    } else {
        println!("Other!!");
    }

    if let Coin::Quarter(state) = c4 {
        println!("Quarter {:?}!!", state);
    } else {
        println!("Other!!");
    }

    println!("{:?}", incr(Some(1)));
    println!("{:?}", incr(None));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}!!", state);
            25
        },
    }
}

fn incr(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
