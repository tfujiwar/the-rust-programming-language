struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop: {}", self.data);
    }
}

fn main() {
    let c1 = CustomSmartPointer { data: String::from("c1") };
    let c2 = CustomSmartPointer { data: String::from("c2") };
    {
        let c3 = CustomSmartPointer { data: String::from("c3") };
        let c3 = CustomSmartPointer { data: String::from("c3 again") };
        println!("inner scope");
    }
    drop(c2);
    println!("outer scope");
}