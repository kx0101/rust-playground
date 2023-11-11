use std::{collections::HashMap, io};

fn main() {
    const DEPARTMENTS: [&str; 2] = ["Engineering", "Sales"];
    let mut map: HashMap<&str, Vec<String>> = HashMap::new();

    while let Some((choice, name)) = get_user_choice(DEPARTMENTS) {
        if name == "stop" {
            break;
        }

        let vector_index = choice - 1;

        let employees = map.entry(DEPARTMENTS[vector_index]).or_insert(Vec::new());

        employees.push(name);
    }

    while let Some(department) = get_user_department(DEPARTMENTS) {
        let employees: Option<&mut Vec<String>> = map.get_mut(department);

        if employees.is_none() {
            println!("\nThere are no employees in this department");
            continue;
        }

        let employees = employees.unwrap();
        employees.sort();

        println!();
        for employee in employees {
            println!("{:#?}", employee);
        }
    }
}

fn get_user_department(departments: [&str; 2]) -> Option<&str> {
    println!("\nChoose a department");

    for (i, department) in departments.iter().enumerate() {
        println!("{}. {}", i + 1, department);
    }

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read choice");

    let choice: usize = choice.trim().parse().expect("failed to parse choice");

    let vector_index = choice - 1;

    Some(departments[vector_index])
}

fn get_user_choice(departments: [&str; 2]) -> Option<(usize, String)> {
    println!("Give me the name of the employee or type 'stop' to move on");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    let name = name.trim().to_string();

    if name == "stop" {
        return None;
    }

    for (i, department) in departments.iter().enumerate() {
        println!("{}. Add {name} to {department}", i + 1);
    }

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read choice");

    let choice: usize = choice.trim().parse().expect("failed to parse choice");

    Some((choice, name))
}
