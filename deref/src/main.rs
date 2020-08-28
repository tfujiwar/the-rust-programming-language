use std::ops::Deref;

fn main() {
    let x = 5;
    let naive_ref = &x;
    let naive_box = Box::new(x);
    let my_box = MyBox::new(x);
    println!("{:?}", x == 5);
    println!("{:?}", *naive_ref == 5);
    println!("{:?}", *naive_box == 5);
    println!("{:?}", *my_box == 5);
    println!("{:?}", *(my_box.deref()) == 5);

    let s = "str";
    hello(&s);

    let s = String::from("String");
    hello(&s);

    let s = MyBox::new(String::from("MyBox"));
    hello(&s);

    println!("MyBox: {:?}", s);
    println!("String: {:?}", s.deref());
    println!("str: {:?}", s.deref().deref());

}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(s: &str) {
    println!("Hello, {}!!", s);
}
