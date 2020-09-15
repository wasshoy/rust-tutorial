fn main() {
    // Copy トレイトが実装されているデータ型 の例その1
    // データそのものはスタックに保持される
    let x = 5; // スタックに x が積まれる
    let y = x; // xの値をコピーした y が新たにスタックに積まれる
    println!("x is {}", x); // x の所有権は移っていないので貸し出し可能
                            // Copy トレイトが実装されているデータ型 の例その2
    let str1 = "hello world";
    let str2 = str1;
    println!("str1 is {}", str1);

    // Copy トレイトが実装されていないデータ型
    // 文字列そのものはヒープ領域に保持される
    // 確保したヒープ領域のポインタ、データが使用しているバイトサイズ、許容量 がスタックに積まれる
    let s1 = String::from("hello world");
    let s2 = s1; // s2にはスタックに保持されたs1のデータがコピーされる（文字列データそのものはコピーしない）
                 // -> ポインタが示す先は同じヒープ領域のデータ
                 // メモリの開放時に二重で開放しないようにするため、所有権を s2 に渡す（ムーブする）
                 // println!("s1 is {}", s1); // s1ムーブされているのでもはや貸し出せない（エラーとなる）

    // let word = firs_tword(&s[..]);
    // let word2 = firs_tword(&s2[..]);
    // println!("word is {}", word);
    // println!("word is {}", word2[..]);
}

fn firs_tword(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
