fn main() {
    let v1 = vec![1, 2, 3];
    let it = v1.iter().map(|x| {
        println!("{}", x);
        x + 100
    });
    for i in it {
        println!("{}", i);
    }

    let c = Counter::new(5);
    for i in c {
        println!("{}", i);
    }
}

struct Counter {
    max: u32,
    count: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { max, count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}