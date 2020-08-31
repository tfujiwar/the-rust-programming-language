#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(1));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    {
        let mut v1 = value.borrow_mut();
        *v1 += 10;
        drop(v1);

        let mut v1 = value.borrow_mut();
        *v1 += 10;
    }
    {
        let mut v2 = value.borrow_mut();
        *v2 += 10;
    }
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
