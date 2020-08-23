fn main() {
    let mut user1 = User {
        name: String::from("tfujiwar"),
        age: 30,
    };
    println!("{} ({})", user1.name, user1.age);

    user1.age = user1.age + 1;
    println!("{} ({})", user1.name, user1.age);

    let user2 = User {
        age: 40,
        ..user1
    };
    println!("{} ({})", user2.name, user2.age);

    let c = Color(0, 0, 255);
    println!("{}, {}, {}", c.0, c.1, c.2);
}

struct User {
    name: String,
    age: u32,
}

struct Color(i32, i32, i32);
