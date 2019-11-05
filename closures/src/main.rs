use std::cmp;
use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, V>
where
    T: Fn(V) -> V,
    V: cmp::Eq + hash::Hash + Copy,
{
    calculation: T,
    value: HashMap<V, V>,
}

impl<T, V> Cacher<T, V>
where
    T: Fn(V) -> V,
    V: cmp::Eq + hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today to {} pushups!", expensive_result.value(intensity));
        println!("Next to {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_u32_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_str_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value('a');
    let v2 = c.value('b');

    assert_eq!(v2, 'b');
}

fn main() {
    generate_workout(10, 3);
    generate_workout(26, 10);
}
