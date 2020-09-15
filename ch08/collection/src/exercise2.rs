fn main() {
    // 練習問題
    // ピッグラテン文字列への変換
    let texts = vec![
        String::from("first"),
        String::from("apple"),
        String::from("こんにちは"),
    ];
    for text in &texts {
        let mut i = 0;
        let mut pig = String::new();
        let mut first = String::new();
        let mut is_start_vowel: bool = false;
        for c in text.chars() {
            if i != 0 {
                pig += &c.to_string();
            } else {
                if c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o' {
                    is_start_vowel = true;
                    break;
                } else {
                    is_start_vowel = false;
                    first += &c.to_string();
                }
            }
            i += 1;
        }
        pig = if is_start_vowel {
            format!("{}-{}", &text, "hay")
        } else {
            format!("{}-{}{}", &pig, &first, "ay")
        };
        println!("{} => ピッグ・ラテン文字列 : {}", &text, &pig);
    }
}
