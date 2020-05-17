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
    // この状態だと，引数を変えて計算し直しができない点に注意したほうがいい
    // Hashマップの保持をするとかしよう
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

    // いつもc++でやるような，関数の引数にclosureを与えるパターン
    // あえて渡す必要のない例かもしれないが
    fn generate_workout_with_closure_improved<F: Fn(u32) -> u32>(
        intensity: u32,
        random_number: u32,
        expensive_function: F,
    ) {
        let expensive_result = expensive_function(intensity);
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

    pub fn run_with_closure_improved() {
        println!("generate with closure improved");
        let random_value = rand::thread_rng().gen_range(1, 5);
        let expensive_function = |intensity: u32| -> u32 {
            println!("calculating slowly...");
            std::thread::sleep(std::time::Duration::from_secs(2));
            intensity
        };
        generate_workout_with_closure_improved(26, random_value, expensive_function);
        generate_workout_with_closure_improved(25, random_value, expensive_function);
    }

    // 値を不変で借用する
    // Fnトレイトの性質
    pub fn fn_trait() {
        // Copyトレイトあり
        let x = 3;
        let y = 3;
        let equal_to_x = |z| z == x;
        let _result = equal_to_x(y);
        println!("x is alive: {}", x);

        // Copyトレイトなし
        let a = vec![1, 2, 3];
        let b = vec![1, 3, 2];
        // aを不変借用
        let equal_to_a = |c| c == a;
        let _result = equal_to_a(b);
        println!("a is alive: {:?}", a);
    }

    // 値の所有権を奪う
    // FnOnceトレイトの性質
    pub fn fn_once_trait() {
        // Copyトレイトあり
        let x = 2;
        let y = 3;
        let equal_to_x = move |z| z == x;
        let _result = equal_to_x(y);
        // Dropないから大丈夫
        println!("x is alive: {}", x);

        // Copyトレイトなし
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        // aの所有権を奪う
        let equal_to_a = move |c| c == a;
        // aはもういない
        // println!("a is alive: {:?}", a);
        println!("a is dead:");
        // 一度Callすると消費されるので，二回目は呼ばれない
        let _result = equal_to_a(b);
    }
}

pub mod iterators {
    // これまでの例で出てきた流れの再掲
    pub fn simple_iteration() {
        let v1 = vec![1, 2, 3];

        // イテレータは怠惰である，と表現されている
        // それを使って何かをしない限りは，特に仕事をしないということ
        // 使われてなんぼ
        let v1_it = v1.iter();

        // * 所有権移動します
        // forでiterを使うときにmutにする必要がないのは...まぁ後でいいや
        for val in v1_it {
            println!("simple_iteration: {}", val);
        }

        // 使えません
        // nextを使うときはv1_itはmutにしてね
        // println!("{}", v1_it.next().unwrap());
    }

    // 手動で要素のスキャンをする
    pub fn next() {
        let v1 = vec![1, 2, 3];

        // next()を使うならmutにすべし
        let mut v1_it = v1.iter();

        println!("{}", v1_it.next().unwrap()); // 1
        println!("{}", v1_it.next().unwrap()); // 2
        println!("{}", v1_it.next().unwrap()); // 3

        // none value
        // println!("{}", v1_it.next().unwrap());
    }

    // iterは消費アイテム
    // それを使うメソッドによって所有権は移動する
    pub fn sum() {
        let v1 = vec![1, 2, 3];
        let v1_it = v1.iter();

        // iterの所有権は奪われる
        // このステートメントでDropする
        let sum: i32 = v1_it.sum();

        // なので使えませんよ
        // nextを使うときはv1_itはmutにしてね
        // println!("{}", v1_it.next().unwrap());

        println!("use_sum: sum is {}", sum);
    }

    pub fn map() {
        let v1 = vec![1, 2, 3];
        // mapはクロージャをとる
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        println!("mapped v1 is: {:?}", v2);
    }

    pub fn filter() {
        let v1 = vec![1, 2, 3];
        // filterは論理値を返すクロージャをとる
        let v2: Vec<_> = v1.iter().filter(|x| *x % 2 == 0).collect();

        println!("filtered v1 is: {:?}", v2);
    }
}
