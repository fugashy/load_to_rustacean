pub mod closures {
    extern crate rand;
    use rand::Rng;

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        intensity
    }

    fn generate_workout_without_closure(intensity: u32, random_number: u32) {
        if intensity > 25 {
            println!(
                "Today, do {} pushups",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stray hydrated");
            } else {
                println!(
                    "Today, run for {} minites",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }

    pub fn run_without_closure() {
        println!("generate without closure");
        let random_value = rand::thread_rng().gen_range(1, 5);
        generate_workout_without_closure(26, random_value);
        generate_workout_without_closure(25, random_value);
    }

    fn generate_workout_with_closure(intensity: u32, random_number: u32) {
        let expensive_closure = |intensity: u32| -> u32 {
            println!("calculating slowly...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };

        if intensity > 25 {
            println!("Today, do {} pushups", expensive_closure(intensity));
            println!("Next, do {} situps", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stray hydrated");
            } else {
                println!("Today, run for {} minites", expensive_closure(intensity));
            }
        }
    }

    pub fn run_with_closure() {
        println!("generate with closure");
        let random_value = rand::thread_rng().gen_range(1, 5);
        generate_workout_with_closure(26, random_value);
        generate_workout_with_closure(25, random_value);
    }
}

#[cfg(test)]
mod unit_tests {

    use super::closures;

    mod simulated_expensive_calculation {
        #[test]
        fn success() {
            assert_eq!(super::closures::simulated_expensive_calculation(3), 3);
        }
    }
}
