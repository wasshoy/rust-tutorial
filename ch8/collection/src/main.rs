fn main() {
    // ベクタ型 Vec<T>
    // let v: Vec<i32> = Vec::new();  // 不変な空の初期化
    // let v = vec![1, 2, 3]; // 可変な初期化
    // v.push(5);
    // v.push(6);
    // println!("{:?}", v);
    // let v2: &i32 = &v[100];  // panic を起こす
    // println!("v2 is {:?}", v2);
    // let v2: Option<&i32> = v.get(100); // None を返す
    // println!("v2 is {:?}", v2);

    // 借用に関するエラーが起きる（なぜか起きない）
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];
    // v.push(6);

    // ベクタ型の中身を加工
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50; // * で参照外しをして値にアクセス
    //     println!("{}", i);
    // }
    // println!("v is {:?}", v);

    // 文字列
    // let mut s = String::new(); // 可変な空の初期化
    // 文字列リテラルからの初期化の方法（どちらも等価）
    // let s = "initial contents".to_string(); // 文字列リテラルから to_string メソッドで String の初期化
    // let s = String::from("initial contents"); // 文字列リテラルから String::from メソッドで String の初期化

    let mut s = String::from("こんにちは");
    s.push_str("世界"); // 末尾に文字列スライスを追加
    println!("Hello, world! -> {}", s);
}
