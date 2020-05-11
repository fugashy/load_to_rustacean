pub mod closures {
    extern crate rand;
    use rand::Rng;

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        intensity
    }

    #[test]
    fn simulated_expensive_calculation_success() {
        assert_eq!(super::closures::simulated_expensive_calculation(3), 3);
    }

    fn generate_workout_without_closure(intensity: u32, random_number: u32) {
        let expensive_result = simulated_expensive_calculation(intensity);
        if intensity > 25 {
            println!("Today, do {} pushups", expensive_result);
            println!("Next, do {} situps", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stray hydrated");
            } else {
                println!("Today, run for {} minites", expensive_result);
            }
        }
    }

    pub fn run_without_closure() {
        println!("generate without closure");
        let random_value = rand::thread_rng().gen_range(1, 5);
        generate_workout_without_closure(26, random_value);
        generate_workout_without_closure(25, random_value);
    }

    // closure本体と呼び出し結果を保存する構造体を作る
    // closureはFnトレイト，FnMut，FnOnceトレイトのどれかを実装している
    // 今回はFnを使ってみる
    // トレイト境界を見るとclosureだなと，rustマンにはわかる
    struct Cacher<T>
    where
        T: Fn(u32) -> u32, // 引数x1でその引数と同じ型の値を返すclosureであることが一目瞭然
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                // 何もなかったときのみclosureをcallする
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    #[test]
    fn expected_behaivior_of_cacher() {
        let mut c = super::closures::Cacher::new(|n| n);
        assert_eq!(c.value(3), 3);
    }

    fn generate_workout_with_closure(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|intensity: u32| -> u32 {
            println!("calculating slowly...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        });

        if intensity > 25 {
            println!("Today, do {} pushups", expensive_result.value(intensity));
            println!("Next, do {} situps", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stray hydrated");
            } else {
                println!(
                    "Today, run for {} minites",
                    expensive_result.value(intensity)
                );
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
