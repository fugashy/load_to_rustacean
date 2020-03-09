// String型を例に所有権について理解していく
//
// 文字リテラルはコンパイル時に中身がわかっているのでバイナリにハードコードされる
// そのため高速で効率的
//
// String型は可変で伸長でき，破片のサポートするため，コンパリル時には不明な量のメモリをヒープに格納する
// ヒープはトラブルの種であり，これを管理することが所有権の存在理由となっている

fn scope_lieral() {
    {
        // sはこのスコープの中のみで有効
        let s = "hello";
        println!("{}", s);
    }
    // cannot find value `s` in this scope
    // println!("{}", s);
}

fn sample_of_string_type() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
}

fn interactions_of_variables_and_data_in_stack() {
    // 値5をxに束縛する
    let x = 5;
    // xの値をコピーしてyに束縛する
    // これらの値は固定長で，スタックに積まれるため可能
    let y = x;

    println!("I want to show value of x: {}", x);
    println!("I want to show value of y: {}", y);
}

fn interactions_of_variables_and_data_in_heap() {
    // String型は
    // - 文字列の中身を保持するメモリへのポインタ(stack)
    // - 中身(heap)
    // - その長さ(stack)
    // - 許容量(stack)
    // でできている
    let s1 = String::from("hello");
    // コピーされるのはstackのデータのみ
    // というより，コピーではなくmoveされる
    // この時点でs1は有効な所有者ではなくなる
    // heapのデータもコピーしていたら，大容量の場合非効率なのでrustではしない
    let s2 = s1;
    // エラー
    // println!("I want to show value of s1: {}", s1);
    // OK
    println!("I want to show value of s2: {}", s2);

    // cloneなら，heapを使う所有者が持つ情報を全てコピーできる
    let s3 = s2.clone();
    // 借用されなくなる
    println!("I want to show value of s2: {}", s2);
    // s2のコピーになれる
    println!("I want to show value of s3: {}", s3);
}

fn takes_onwership(some_string: String) {
    // some_stringがスコープに入る
    println!("some string is: {}", some_string);
    // dropが呼ばれ，メモリが開放される
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_stringを返す(move)
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // スコープにはいるが，そのまま返す(move)
    a_string
}

fn makes_copy(some_integer: i32) {
    // some_integerのスコープ
    println!("some integer is: {}", some_integer);
    // 特に何もなし
}

fn test_ownership() {
    let s = String::from("hello");
    // この関数で所有権が譲渡される
    takes_onwership(s);
    // error !
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    println!("x is: {}", x);

    // 返り値の所有権を得た
    let s = gives_ownership();
    println!("gave {}", s);
    // 渡して，すぐ帰ってきた
    let s = takes_and_gives_back(s);
    println!("take and give {}", s);
}

// 参照型のStringが引数
fn calculate_length(s: &String) -> usize {
    s.len()
    // sは参照型なので，スコープを抜けてもdropされるわけではない
}

// 所有権を得てはすぐ返すのをやたらと実装するのはめんどくさい
// 参照という仕組みも用意されている
fn reference_sample() {
    let s1 = String::from("hello");
    // 参照型の関数には参照を渡すことを明示する
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}

// 可変参照をシグネチャにする
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // some_stringは参照型なので，スコープを抜けてもdropされるわけではない
}

fn modify_referenced_variable() {
    // 可変の変数を
    let mut s = String::from("hello");

    // 可変参照する
    change(&mut s);
    // dropされないので使える
    println!("{}", s);
}

fn double_reference() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s;
    // cannot borrow `s` as immutable because it is also borrowed as mutable
    // let r3 = &s;

    println!("{}", r1);
    // println!("{}", r2);
    // println!("{}", r3);
}

fn main() {
    scope_lieral();
    sample_of_string_type();
    interactions_of_variables_and_data_in_stack();
    interactions_of_variables_and_data_in_heap();
    test_ownership();
    reference_sample();
    modify_referenced_variable();
    double_reference();
}
