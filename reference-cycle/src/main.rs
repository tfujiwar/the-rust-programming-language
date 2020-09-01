use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));

    println!("{}", Rc::strong_count(&a));
    println!("{:?}", a);
    println!("{:?}", a.tail());

    println!("{}", Rc::strong_count(&b));
    println!("{:?}", b);
    println!("{:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("{}", Rc::strong_count(&a));
    println!("{}", Rc::strong_count(&b));
    // println!("{:?}", a);
}
