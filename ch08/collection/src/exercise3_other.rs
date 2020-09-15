fn main() {
    // 練習問題の別解
    // 構造体とベクタを使用したテキストインターフェイスの作成

    use std::io;

    let mut departments: Vec<String> = Vec::new();
    let mut employees: Vec<Employee> = Vec::new();

    println!("Start. (help: Help, quit: ctrl + C.)");
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Faild to read line.");

        let command: Vec<&str> = command.trim().split_whitespace().collect();
        if command.len() == 0 {
            continue;
        } else {
            match command[0] {
                "Add" => {
                    if command.len() != 4 {
                        println!("can't add.\nIf you add someone, write as below.\nAdd [name] to [department]");
                    } else {
                        let name = command[1].to_string();
                        let department = command[3].to_string();
                        if !is_registerd(&departments, &department) {
                            departments.push(department.to_string());
                        }

                        println!("{} is registerd as {}.", &name, &department);
                        let employee = Employee {
                            name: name,
                            department: department,
                        };
                        employees.push(employee);
                    }
                }
                "List" => {
                    if command.len() == 1 {
                        for e in &employees {
                            println!("{}({})", &e.name, &e.department);
                        }
                    } else {
                        let department = command[1];
                        if !is_registerd(&departments, &department.to_string()) {
                            println!("No one is registerd as {}.", &department);
                        } else {
                            for e in &employees {
                                if e.department == department {
                                    println!("{}", &e.name);
                                }
                            }
                        }
                    }
                }
                "Help" => {
                    println!("You can use below commands.\n--");
                    println!("Add [name] to [department] : register someone to some department.");
                    println!("List: show all employees orderded by department.");
                    println!("List [department]: show all employees belong to some department.");
                    println!("Help: show all commands.\n--\n");
                }
                _ => println!("command not founded."),
            }
        }
    }
}

struct Employee {
    name: String,
    department: String,
}

fn is_registerd(departments: &Vec<String>, department: &String) -> bool {
    for d in departments {
        if d == department {
            return true;
        };
    }
    false
}
