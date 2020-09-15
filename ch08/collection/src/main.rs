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

    // let mut s = String::from("こんにちは");
    // s.push_str("世界"); // 末尾に文字列スライスを追加
    // println!("Hello, world! -> {}", s);

    // let mut s1 = "foo".to_string();
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // // 文字列の追加方法 1
    // // let s3 = s1 + &s2; // s1 はムーブされるので今後使えない
    // // 文字列の追加方法 2
    // let s3 = String::from("fuga");
    // let s4 = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s4);

    // // let s4_0 = s4[0]; // 文字列は添字でアクセスできないようになっている

    // println!(
    //     "{} の長さは {}",
    //     String::from("hola"),
    //     String::from("hola").len()
    // );
    // println!(
    //     "{} の長さは {}",
    //     String::from("こんにちは"),
    //     String::from("こんにちは").len()
    // ); // 長さは15になる  ひらがな1文字はUnicodeスカラー値(=Unicodeのコードポイントでの値)で 3バイトで表現されるため

    // // スライスして確かめる
    // let kon = String::from("こんにちは");
    // println!("こんにちはの1文字目ををスライス : {}", &kon[0..3]);
    // // &kon[0..4] などするとエラー（「こ」と「ん」の1バイト目が混在するため、文字の境界でないと言われる）

    // // １文字ずつ取り出す最適な方法 : charsメソッドを使う
    // for c in kon.chars() {
    //     // cにはchar型の値が入る
    //     println!("{}", c);
    // }
    // // 各文字のバイト
    // for b in kon.bytes() {
    //     // 15バイトの値がそれぞれ1つずつbに入る
    //     println!("{}", b);
    // }

    //ハッシュマップ  Rustの連想配列
    // use std::collections::HashMap;

    // ハッシュマップを新たに生成
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // 2つのタプルからハッシュマップを生成
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // // 型注釈はコンパイラに推論させる
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // // ハッシュマップと所有権
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // // println!("{}, {}", field_name, field_value);  // 所有権がmapに移っている

    // // ハッシュマップの値にアクセスする
    // let mut scores = HashMap::new();

    // // 値の挿入
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name); // 返り値の型はOption<&V> 無ければNoneを返すようにするため
    // println!("{:?}", score);
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // // ハッシュマップの更新
    // // 値の上書き
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores); // 25に更新されている

    // // キーに値がまだなかったときのみ挿入  entryメソッドとor_insertメソッドを使う
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50); // entryは値が存在するかどうかを返す or_insertを使い、無ければ挿入(あれば値に対する可変参照を返す)
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores); // Blue は更新されていない

    // // 元々ある値をもとに更新する

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();
    // // 単語の出現回数をそれぞれカウント
    // for word in text.split_whitespace() {
    //     // 初めて見る単語は0で初期化
    //     let count = map.entry(word).or_insert(0); // 値が存在すれば可変参照を返す
    //     *count += 1; // 参照外しをして1を足す
    // }

    // println!("{:?}", map);

    // 演習はexercise#.rs として別ファイルに記述
}
