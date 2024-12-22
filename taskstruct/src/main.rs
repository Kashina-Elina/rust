use std::collections::HashMap;
use std::io::{self, Write};

fn add_to_vector(vec: &mut Vec<String>, name: String) {
    vec.push(name); 
}

fn main() {
    let mut company = HashMap::new();
    let mut people_in_engineering: Vec<String> = Vec::new();
    let mut people_in_sales: Vec<String> = Vec::new();

    people_in_engineering.push("Alice".to_string());
    people_in_sales.push("Kate".to_string());

    company.insert("Engineering".to_string(), people_in_engineering);
    company.insert("Sales".to_string(), people_in_sales);

    loop {
    print!("Введите команду Add name to department или List department или People in company или Quit ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 

    let input = input.trim();

    if input.to_lowercase().starts_with("add ") {
        let parts: Vec<&str> = input[4..].split_whitespace().collect();

        if parts.len() < 3 {
            println!("Ошибка: Не указано имя или отдел.");
        } else {
            let name = parts[0]; 
            let department = parts[2];

            if let Some(department_employees) = company.get_mut(department) {
                add_to_vector(department_employees, name.to_string());
                println!("{} добавлен в отдел {}!", name, department);
            } else {
                println!("Ошибка: Отдел с названием '{}' не найден.", department);
            }
        }
    } else if input.to_lowercase().starts_with("list "){
        let parts: Vec<&str> = input[5..].split_whitespace().collect();

        if parts.len() < 1 {
            println!("Ошибка: Не указан отдел.");
        } else {
            let department = parts[0]; 

            if let Some(department_employees) = company.get_mut(department) {
                match company.get(department) {
                    Some(employees) => { println!("Сотрудники отдела {}: {:?}", department, employees);}
                    None => todo!()
        }
            } else {
                println!("Ошибка: Отдел с названием '{}' не найден.", department);
            }
        }
    }
    else if input.to_lowercase().starts_with("people"){
        let mut sorted_departments: Vec<_> = company.iter().collect();
            sorted_departments.sort_by_key(|&(department, _)| department);
            for (department, employees) in sorted_departments {
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                println!("Отдел {}: {:?}", department, sorted_employees);
            }
    }
    else if input.to_lowercase().starts_with("quit"){
        break;
        }
    
}
}
