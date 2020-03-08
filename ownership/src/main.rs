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
}

fn main() {
    scope_lieral();
    sample_of_string_type();
}
