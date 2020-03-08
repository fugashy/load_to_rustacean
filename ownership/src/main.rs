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

fn main() {
    scope_lieral();
    sample_of_string_type();
    interactions_of_variables_and_data_in_stack();
    interactions_of_variables_and_data_in_heap();
}
