use std::collections::HashMap;
use std::io;
fn main() {
    // 練習問題
    // ハッシュマップとベクタを使用したテキストインターフェイスの作成

    let mut employees: HashMap<String, Vec<String>> = HashMap::new(); // キーが部署、値が所属する従業員の名前の可変長配列

    // テストのための初期化 TODO: 完成後削除
    // employees.insert(String::from("wasshoy"), String::from("Sales"));

    println!("Start. (\"help\" でコマンド一覧を表示)");
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Faild to read line.");

        if !parse_command(&s.trim(), &mut employees) {
            // quit が入力されると終了
            break;
        }
    }
}

fn parse_command(s: &str, employees: &mut HashMap<String, Vec<String>>) -> bool {
    let commands: Vec<&str> = s.trim().split(" ").collect();
    match commands[0] {
        "add" if commands.len() == 3 => {
            add_employee(employees, commands[1], commands[2]);
            true
        }
        "list" if 1 == commands.len() => {
            println!("{:?}", employees);
            true
        }
        "list" if 1 < commands.len() => {
            println!("{:?}", employees.get(commands[1]));
            true
        }
        "quit" => false,
        "help" => {
            println!("you can use commands below.");
            println!("----");
            println!("add [部署名] [従業員名] : 従業員を登録する。");
            println!("list : 従業員の一覧を表示する。");
            println!("list [部署名] : ある部署の従業員の一覧を表示する。");
            println!("help : コマンド一覧を表示する。");
            println!("quit : 終了する。");
            println!("----");
            true
        }
        _ => true,
    }
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    employees
        .entry(department.to_string())
        .or_insert_with(|| Vec::new())
        .push(employee.to_string());
}
