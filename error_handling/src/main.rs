use std::fs::File;
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

fn try_to_read_file_then_handling() {
    let filename = String::from("/tmp/hello.txt");
    let _f = File::open(&filename);

    let _f = match _f {
        Ok(file) => file,
        // 見つからない場合は作ってみる
        // if ...はマッチガードと呼ばれる
        // TODO(fugashy) &ではなくrefを使用する理由は後ほど
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
            match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create a file but there was a problem: {:?}", e),
            }
        }
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn main() {
    // simple_panic();
    // try_to_buffer_overread();
    try_to_read_file_then_handling();
}
