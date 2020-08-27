#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 2, height: 2};
        let smaller = Rectangle { width: 1, height: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 2, height: 2};
        let smaller = Rectangle { width: 1, height: 1};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "this is a message")]
    fn fail() {
        assert!(false, "this is a message");
    }
}
