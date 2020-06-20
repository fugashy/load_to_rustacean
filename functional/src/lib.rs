// クロージャのまとめ
// - C++と大体一緒
// -- キャプチャ付きの匿名関数
// 下記が特徴的(C++比)
// - キャプチャ対象にも所有権の仕組みが適応されること
// - キャプチャの方式毎にクロージャの型が存在すること
// -- Fn
// -- FnOnce
// -- FnMut
pub mod closures {
    extern crate rand;
    use rand::Rng;
    use std::collections::HashMap;

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

    // 応用例：関数と，その結果のキャッシュを持つ構造体
    // 重たい関数の場合役立つ
    // トレイト境界を見るとclosureだなと，rustマンにはわかる
    struct Cacher<T>
    where
        T: Fn(u32) -> u32, // 引数x1でその引数と同じ型の値を返すclosureであることが一目瞭然
    {
        calculation: T,
        value_by_arg: HashMap<u32, u32>, // 引き数毎に結果を格納する
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value_by_arg: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            // or_insertは&mut u32を返してくる
            // consider dereferencing the borrow
            // デリファレンスして，値をHashmapの生存期間の外へ出す(という解釈で良いのだろうか)
            *self
                .value_by_arg
                .entry(arg) // このキーは存在するのか
                .or_insert((self.calculation)(arg)) // もしなければ追加する
                                                    // to call the function stored in `calculation`,
                                                    // surround the field access with parentheses
        }
    }

    #[test]
    fn expected_behaivior_of_cacher() {
        let mut c = super::closures::Cacher::new(|n| n + 1);
        // 設定したクロージャのアウトプットを返してくれる
        assert_eq!(c.value(3), 4);
        // ハッシュマップに登録されてない引き数を与えても大丈夫
        assert_eq!(c.value(2), 3);
        // 再現性を一応
        assert_eq!(c.value(2), 3);
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

    // いろんな書き方
    // 引き数 + 1 を計算するシンプルなものを使った例
    pub fn several_ways_to_describe_closures() {
        // 明示的に書く例
        // わかりやすい
        let c1 = |x: u32| -> u32 { x + 1 };

        // rustfmtなどの整形ツールを使うと{}は自動で消えてしまう
        // あえて{}を書いてあげるほど複雑ではないからかもしれない
        // let c2 = |x| {x + 1};

        // 最小限の例
        // シンプルなものならこっちがよい
        let c3 = |x| x + 1;

        println!("c1(3) is: {}", c1(3)); // 4

        // println!("c2(4) is: {}", c2(4)); // 5

        println!("c3(5) is: {}", c3(5)); // 6
    }

    // 値を不変で借用する形でキャプチャする
    pub fn capture_values_as_const_one() {
        // Copyトレイトあり
        let x = 3;
        let y = 3;
        let equal_to_x = |z| z == x;
        let _result = equal_to_x(y);
        println!("x is alive: {}", x); // 3

        // Copyトレイトなし
        let a = vec![1, 2, 3];
        let b = vec![1, 3, 2];
        // aを不変借用
        let equal_to_a = |c| c == a;
        let result = equal_to_a(b); // 借用しているだけなので所有権は移動していない
        println!("a is alive: {:?}", a); // なので使用可能

        println!("result: {:?}", result); // false
    }

    // 値の所有権を奪うことを強制するmoveキーワード
    pub fn capture_values_in_whole() {
        // Copyトレイトあり
        let x = 2;
        let y = 3;
        let equal_to_x = move |z| z == x;
        let _result = equal_to_x(y);
        // Dropないから大丈夫
        println!("x is alive: {}", x);
        // 2回呼べる
        let _result = equal_to_x(y);
        println!("x is alive, again: {}", x);

        // Copyトレイトなし
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        // moveキーワードでaの所有権を奪う
        // ==はeq(&self, other: &Self) -> bool
        let equal_to_a = move |c| c == a; // variable 'a' moved due to use in closure

        // aはもういない. 下のprintlnは怒られる
        // println!("a is alive: {:?}", a);

        let result = equal_to_a(b); // variable 'b' moved due to use in closure
        println!("result: {:?}", result); // true
    }

    // 所有権を奪った先で消化してみる
    pub fn capture_values_in_whole_2() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        // moveキーワードでaの所有権を奪う
        let equal_to_a =
            // into_iter()でvecの所有権を奪い，sumで消化仕切る
            move |c: Vec<i32>| c.into_iter().sum::<i32>() == a.into_iter().sum::<i32>(); // variable 'a' moved due to use in closure

        // 1回目
        let result = equal_to_a(b); // variable 'b' moved due to use in closure
                                    // aは消費される(drop)
        println!("result: {:?}", result); // true

        // 2回目
        // 怒られる
        // let c = vec![3, 2, 1];
        // let result = equal_to_a(c); // variable 'c' moved due to use in closure
        //                             // aはもういない
        // println!("result: {:?}", result); // true
    }
}

// iteratorまとめ
// - 構造はC++と似ている
pub mod iterators {
    use std::fmt;

    // これまでの例で出てきた流れの再掲
    pub fn simple_iteration() {
        let v1 = vec![1, 2, 3];

        // イテレータは怠惰である，と表現されている
        // それを使って何かをしない限りは，特に仕事をしないということ
        // 使われてなんぼ
        let v1_it = v1.iter();

        // v1_itの所有権は移動され，Dropされます
        for val in v1_it {
            println!("simple_iteration: {}", val);
        }

        // なので使えません
        // println!("{}", v1_it.sum::<i32>());
    }

    // 2D配列のスキャンもc++, python間隔で可能
    pub fn double_iteration() {
        let mat_int = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        for row in mat_int.iter() {
            for col in row.iter() {
                println!("the element is: {}", col); // 1, 2, 3, ..., 9
            }
        }

        let mat_string = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")],
        ];

        for row in mat_string.iter() {
            for col in row.iter() {
                println!("the element is: {}", col); // a, b, c, ..., i
            }
        }
    }

    // 手動で要素のスキャンをする
    pub fn next() {
        let v1 = vec![1, 2, 3];

        // next()を使うならmutにすべし
        // 内部変数に変更が行われるため
        let mut v1_it = v1.iter();

        println!("{}", v1_it.next().unwrap()); // 1
        println!("{}", v1_it.next().unwrap()); // 2
        println!("{}", v1_it.next().unwrap()); // 3

        // none value
        // println!("{}", v1_it.next().unwrap());
    }

    // 総和を計算するメソッド
    // 所有権を奪う
    pub fn sum() {
        let v1 = vec![1, 2, 3];
        let v1_it = v1.iter();

        // sumメソッドでiterの所有権は奪われる
        // fn sum<S>(self) -> S
        // v1_itはこの文でDropされる
        let sum: i32 = v1_it.sum();
        println!("use_sum: sum is {}", sum); // 6

        // なので使えませんよ
        // nextを使うときはv1_itはmutにしてね
        // println!("{}", v1_it.next().unwrap());

        // v1の所有権までは取らないよ
        println!("v1: {:?}", v1);
    }

    // Iteratorトレイトの便利関数の一部を紹介
    // その他はここ-> https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // 任意のクロージャでコレクションを変換できる便利なやつ
    pub fn map() {
        let v1 = vec![1, 2, 3];
        // mapはクロージャをとる
        // collectでVecにする
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        println!("mapped v1 is: {:?}", v2); // [2, 3, 4]
    }

    // 任意の判定式でコレクションを抽出する便利なやつ
    pub fn filter() {
        let v1 = vec![1, 2, 3];
        // filterは論理値を返すクロージャをとる
        let v2: Vec<_> = v1.iter().filter(|x| *x % 2 == 0).collect();

        println!("filtered v1 is: {:?}", v2); // [2]
    }

    // 1~5をカウントするだけのイテレータクラス
    struct Counter {
        count: u32,
    }
    impl Counter {
        // かならず0から開始する
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    // 内部変数確認用
    impl std::fmt::Display for Counter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "counter: {}", self.count)
        }
    }
    impl Iterator for Counter {
        type Item = u32; // 19章で...

        // other methods are default ones.
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // nextを呼んでみる
    // countメンバも更新されている(念の為みてみた)
    pub fn call_next_of_counter() {
        let mut counter = Counter::new();

        println!("the count is: {}", counter); // 0
        println!("call counter.next: {}", counter.next().unwrap()); // 1
        println!("the count is: {}", counter); // 1
        println!("call counter.next: {}", counter.next().unwrap()); // 2
        println!("the count is: {}", counter); // 2
        println!("call counter.next: {}", counter.next().unwrap()); // 3
        println!("the count is: {}", counter); // 3
        println!("call counter.next: {}", counter.next().unwrap()); // 4
        println!("the count is: {}", counter); // 4
        println!("call counter.next: {}", counter.next().unwrap()); // 5
        println!("the count is: {}", counter); // 5

        // none
        // println!("{}", counter.next().unwrap());
    }

    // rustライクに，複雑な処理を順序よく，わかりやすめに書く
    // Iteratorトレイトを実装したため，実装していないその他のデフォルトメソッドが使用可能に
    // その他のメソッドも，nextを使っているため
    // これは楽だ
    pub fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new() // 1から開始（1, 2, 3, 4, 5, None, ...)
            .zip(Counter::new().skip(1)) // 1つ飛ばす(2, 3, 4, 5, None, ...）
            .map(|(a, b)| a * b) // (2, 6, 12, 20, None, ...)
            .filter(|x| x % 3 == 0) // (6, 12)
            .sum(); // 18 !
        println!("Complex sample value is: {}", sum); // 18
    }
}
