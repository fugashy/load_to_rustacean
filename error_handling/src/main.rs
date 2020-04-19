/*
 * Rustには2種類のエラー分類がある
 * - 回復可能
 *      Result<T, E>
 * - 回復不能
 *      panic!マクロ
 *          プログラムが失敗のメッセージを表示する
 *          スタックを巻き戻して掃除をする
 *          終了する
 */

#[allow(dead_code)]
fn simple_panic() {
    panic!("crash and burn");
}

// RUST_BACKTRACE=1 cargo runでバックトレースを出力できる
#[allow(dead_code)]
fn try_to_buffer_overread() {
    let v = vec![1, 2, 3];
    println!("v[{}] is: {}", 99, v[99]);
}

fn main() {
    // simple_panic();
    // try_to_buffer_overread();
}
