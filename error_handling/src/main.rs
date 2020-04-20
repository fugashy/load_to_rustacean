use std::fs::File;
use std::io::Read;
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

fn print_type_of<T>(_: T) {
    let type_ = std::any::type_name::<T>();
    println!("The type is: {:?}", type_.to_string());
}

fn try_to_read_file_then_handling() {
    let filename = String::from("/tmp/hello.txt");
    let _f = File::open(&filename);
    // Result<std::fs::File, std::io::error::Error>
    print_type_of(&_f);

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

#[allow(dead_code)]
fn unwrapping() {
    // Okならその中身を，そうでないならpanicする
    let _f = File::open("/tmp/hoge.txt").unwrap();
}

#[allow(dead_code)]
fn expecting() {
    // Okならその中身を，そうでないならpanicし，その時のメッセージを指定できる
    let _f = File::open("/tmp/hoge.txt").expect("Hogeeeeeeeee");
}

// エラー情報をしっかり上位へ返して，エラーハンドリングを移譲しよう
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("/tmp/name.txt");
    let mut f = match f {
        Ok(file) => file,
        // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" }
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        // 空のファイルでも一応成功はする
        // Ok("hogehoge\n")
        Ok(_) => Ok(s),
        // パーミッションからRを抜いたらエラーになった
        // Err(Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
        Err(e) => Err(e),
    }
}

fn read_username_from_file_improved() -> Result<String, std::io::Error> {
    // ?演算子は↑のようなmatch式と同じ働きをする
    // Err(_)時はreturnする点には意識する必要あり
    // Resultを返す関数でしか使用できない点に注意
    let mut f = File::open("/tmp/name.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// かっこいい
fn read_username_from_file_final_form() -> Result<String, std::io::Error> {
    let mut s = String::new();
    // Err時はその時にreturnされるので，後の処理はOk前提でかける
    File::open("/tmp/name.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // simple_panic();
    // try_to_buffer_overread();
    try_to_read_file_then_handling();
    // unwrapping();
    // expecting();
    println!("Result of read_username: {:?}", read_username_from_file());
    println!(
        "Result of read_username improved: {:?}",
        read_username_from_file_improved()
    );
    println!(
        "Result of read_username final: {:?}",
        read_username_from_file_final_form()
    );
}
