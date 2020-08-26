fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotername"),
        ..user1
    };
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    // タプル構造体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black is ({}, {}, {})", black.0, black.1, black.2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "the area of the rectangle is {} square pixels.",
        area(&rect1) // ムーブを防ぐための参照渡し
    );
    println!("rect1 is {:#?}", rect1);
    // メソッド
    println!(
        "the area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

#[derive(Debug)] // Rectangleのインスタンスで {:?} または {:#?} を使えるようにする
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 構造体
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
