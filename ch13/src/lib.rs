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
    let expensive_closure = |intensity| {
        simulated_expensive_calc(intensity)
    };
    if intensity < 25 {
        let expensive_result = expensive_closure(intensity);
        println!("Do {} pushups", expensive_result);
        println!("And now {} situps", expensive_result);
    } else {
        if random_number == 3 {
            println!("Rest day");
        } else {
            println!("Run for {} minutes", expensive_closure(intensity))
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
   
    }
}