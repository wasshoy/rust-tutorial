fn main() {
    // 演習
    // 温度を華氏と摂氏で変換
    let k = 273.15;
    println!("{} Kelvin is {} °C.", k, convert_k_to_c(k));
    println!("{} Kelvin is {} F.", k, convert_k_to_f(k));

    // フィボナッチ数列の n 番目を生成
    let n = 5;
    println!("Fibonacci({}) is {}.", n, fibonacci_n(n));

    // The Twelve Days of Christmas の歌詞を表示
    // 略
}

fn convert_k_to_c(k: f32) -> f32 {
    k - 273.15
}

fn convert_k_to_f(k: f32) -> f32 {
    1.8 * convert_k_to_c(k) + 32.
}

fn fibonacci_n(n: u32) -> u32 {
    let _fib_n: u32 = 0;
    if n < 1 {
        let _fib_n = 0;
    } else if n == 1 || n == 2 {
        let _fib_n = 1;
    } else {
        let _fib_n = fibonacci_n(n - 2) + fibonacci_n(n - 1);
    }
    _fib_n
}
