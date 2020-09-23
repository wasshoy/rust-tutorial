use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);  // random_number == 3のときは計算の必要がない
    println!("トレーニングの準備をします。");
    let mut expensive_result = Cacher::new(|num| {
        println!("時間のかかる計算中...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "今日は{}回腕立て伏せをしましょう！",
            expensive_result.value(intensity) // ここで重い計算が行われる
        );
        println!(
            "次に{}回腹筋をしましょう！",
            expensive_result.value(intensity) // キャッシュされた結果を呼び出すだけで重い計算は行わない
        );
    } else {
        if random_number == 3 {
            println!("今日は休憩しましょう！水分補給を忘れずに！")
        } else {
            println!("次に{}分走りましょう！", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 20;
    let simulated_random_number = 4;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// Fnトレイトが実装され、u32を引数にとりu32を返すクロージャーを引数に取る構造体
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
