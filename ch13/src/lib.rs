use std::marker::PhantomData;
use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub fn run_simulation() {
    let input = 10;
    let random = 6;
    generate_workout(input, random)
}

fn simulated_expensive_calc(intensity: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32,random_number:u32) {

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        
        println!("Do {} pushups", expensive_result.value(intensity));
        println!("And now {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Rest day");
        } else {
            println!("Run for {} minutes", expensive_result.value(intensity))
        }

    }
}

/// Cacher is struct that wraps a closure and caches the evaluation
struct Cacher<T,U,V>
where T: Fn(U) -> V,
{
    calculation:T,
    value:Option<V>,
    phantom: PhantomData<U>,
}


impl<T,U,V:Clone> Cacher<T,U,V> where T:Fn(U)->V {
    fn new(calculation:T) ->Cacher<T,U,V> {
        Cacher { calculation: calculation, value: None, phantom:PhantomData }
    }

    fn value(&mut self, arg: U) -> V {
        match &self.value{
            // cache first result forevs
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher() {
        let closure = |i: u32| {
            i*2
        };
        let mut c = Cacher::new(closure);
        let v = c.value(4);
        assert_eq!(v, 8);
    }
}