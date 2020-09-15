use std::fs::File;
use std::io;
use std::io::Read;
use std::net::IpAddr;
// use std::io::ErrorKind;

fn main() {
    // 回復不能なエラー
    // let v = vec![1, 2, 3]
    // v[99]; // ここでpanic!が呼び出される

    // 回復可能なエラー
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!(
    //             "ファイルを作成しようとしましたが、問題が有りました : {:?}",
    //             e
    //         ),
    //     },
    //     Err(error) => panic!("ファイルを開く際に問題がありました : {:?}", error),
    // };
    // エラー時にパニックするためのショートカット
    // let f = File::open("hello.txt").unwrap(); // OkならOkの中身、Errならpanic!を呼び出す
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); // エラーメッセージを自分で記述できる

    // panic!を使う場面を考える
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// 上の関数のショートカット記述版（完全に同義の処理ではない）
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
