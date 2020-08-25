fn main() {
    let s = String::from("hello world");
    let s2 = "hello world";
    let word = firs_tword(&s[..]);
    let word2 = firs_tword(&s2[..]);
    println!("word is {}", word);
    println!("word is {}", word2[..]);
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
