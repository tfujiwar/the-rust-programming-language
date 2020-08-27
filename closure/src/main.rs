use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, V>
    where T: Fn(&U) -> V,
          U: Eq + Hash,
          V: Copy,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(&U) -> V,
    U: std::cmp::Eq + std::hash::Hash,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_int() {
    let mut c = Cacher::new(|&a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
fn call_with_string() {
    let mut c = Cacher::new(|a: &String| a.len());
    let v1 = c.value(String::from("a"));
    let v2 = c.value(String::from("abc"));
    assert_eq!(v1, 1);
    assert_eq!(v2, 3);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calc = Cacher::new(|&num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("{} pushups!", expensive_calc.value(intensity));
        println!("{} situps!", expensive_calc.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break!");
        } else {
            println!("run for {} minutes!", expensive_calc.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 20;
    let simulated_random_number = 7;
    
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}