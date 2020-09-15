fn main() {
    use std::collections::HashMap;
    // 練習問題
    // 整数リストの平均、中央値、最頻値
    let mut nums = vec![3, 1, 6, 5, 2, 4, 8, 7, 9];
    let n = nums.len();
    let mut sum = 0;
    for i in &nums {
        sum += i
    }
    let mean = sum / n;

    nums.sort();
    let med = if n % 2 == 0 {
        (nums[n / 2] + nums[n / 2 - 1]) / 2
    } else {
        nums[n / 2 + 1]
    };

    let mut freq = HashMap::new();
    for i in &nums {
        let cnt = freq.entry(i).or_insert(0);
        *cnt += 1;
    }
    let mut cnt_vec: Vec<_> = freq.iter().collect();
    cnt_vec.sort_by(|a, b| b.1.cmp(a.1)); // 出現頻度の大きい順に並び替え
    let mode = cnt_vec[0].0;
    println!("配列 : {:?}", &nums);
    println!("平均値(mean) : {}", &mean);
    println!("中央値(median) : {}", &med);
    println!("最頻値(mode) : {}", &mode);
}
