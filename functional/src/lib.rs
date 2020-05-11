pub mod closures {
    pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        intensity
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
