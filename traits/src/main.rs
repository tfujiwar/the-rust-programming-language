fn main() {
    let a = Article {
        text: String::from("this is an article"),
        author: String::from("tfujiwar"),
    };

    let t = Tweet {
        message: String::from("this is a tweet"),
        author: String::from("tfujiwar"),
    };

    println!("{}", a.summarize());
    println!("{}", t.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("default")
    }
}

struct Article {
    text: String,
    author: String,
}

struct Tweet {
    message: String,
    author: String,
}

impl Summary for Article {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} (@{})", self.message, self.author)
    }
}
