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
    // #[test]
    // // 予想値は100以下でなければなりません
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    // #[test]
    // // 予想値は100以下でなければなりません
    // #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    // fn less_than_1() {
    //     Guess::new(0);
    // }
    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    // #[test]
    // fn add_two_and_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // #[ignore] // 無視
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }

    // 非公開関数のテスト
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// 非公開関数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
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

fn prints_and_returns_10(a: i32) -> i32 {
    //{}という値を得た
    println!("I got the value {}", a);
    10
}
