// Using a hash map and vectors, create a text interface to allow a user to add employee names to a
// department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments = HashMap::new();

    loop {
        let mut command = String::new();
        println!("Input your command or 'exit':");

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        command = command.to_lowercase().trim().to_string();

        if command == "exit" {
            break;
        }

        if command.starts_with("add") {
            let actions: Vec<&str> = command.split("add").collect();
            let action = actions[1].trim().to_string();
            let to_position = action.find(" to ");
            match to_position {
                Some(pos) => {
                    let nd: Vec<&str> = action.split(" to ").collect();
                    let e = nd[0];
                    let d = nd[1];
                    departments
                        .entry(String::from(d))
                        .or_insert_with(Vec::new)
                        .push(String::from(e));
                    //*employees.push(e);
                    //println!("Adding {} to {}", name_department[0], name_department[1]);
                    //departments_employees
                    //.entry(name_department[1])
                    //.insert(name_department[1], vec![name_department[0]]);
                    println!("{:?}", departments);
                }
                None => println!("Incorrect action!"),
            }
        }
    }
    println!("Terminating.");
}
