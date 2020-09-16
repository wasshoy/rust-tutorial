fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// トレイト境界を使った関数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // トレイト境界は whereを使って書くことも出来る
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// トレイト定義
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(もっと読む)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// トレイトの実装
impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// トレイト境界
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    // ジェネリック
    // let pi = Point { x: 5, y: 10 };

    // let number_list = vec![1, 1, 2, 3, 5, 8];
    // let result = largest(&number_list);
    // println!("最大値は{}です。", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("最大値は{}です。", result);
    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //トレイト
    // let tweet = Tweet {
    //     username: String::from("ロックマン"),
    //     content: String::from("私は人間であって人間でありません。"),
    //     reply: false,
    //     retweet: false,
    // };
    // println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     // ピッツバーグ、ペンシルベニア州、アメリカ
    //     location: String::from("Pittsburgh, PA, USA"),
    //     // アイスバーグ
    //     author: String::from("Iceburgh"),
    //     // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best
    //     hockey team in the NHL.",
    //     ),
    // };
    // // 新しい記事が利用可能です！ {}
    // println!("New article available! {}", article.summarize());

    // ジェネリック
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }  // ここで x は有効ではなくなる
    // println!("r: {}", r); // 借用に関するエラー

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // 最長の文字列は、{}です
    println!("The longest string is {}", result);

    // 異なるライフタイム
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // このスコープで有効
        println!("The longest string is {}", result);
    }

    // 静的ライフタイム
    let s: &'static str = "静的ライフタイムを持ってます。";
}
// ジェネリックな型引数、トレイト境界、ライフタイムまとめ
use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("アナウンス! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ライフタイム省略
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
