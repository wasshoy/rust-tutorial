#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "挨拶に名前が含まれていません。値は {}",
    //         result
    //     ); // 出力に名前がちゃんと出現しているか
    // }

    // パニックが起きたらテストが通る
    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    // パニックが起きた場合のメッセージに含まれていてほしいメッセージを確認し、想定したパニックが起きているかテストする
    #[test]
    // 予想値は100以下でなければなりません
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // 予想値は100以下でなければなりません
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    // format!("こんにちは、{}!", name)
    String::from("こんにちは!")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            //予想値は、1以上でなければなりませんが、{}でした
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            //予想値は100以下でなければなりませんが、{}でした
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}
