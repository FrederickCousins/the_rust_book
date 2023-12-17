use std::{
    collections::HashMap,
    io::{self, Write, Read},
    process::exit,
};

pub fn run() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();
    db.insert(String::from("Sales"), ["John Doe", "Jane Doe"].iter().map(|s| s.to_string()).collect());
    db.insert(String::from("HR"), ["Mohammed Ali"].iter().map(|s| s.to_string()).collect());

    if prerun() {
        interface(&mut db);
    } else {
        println!("Exiting");
    }
}

fn prerun() -> bool {
    println!("Would you like to enter the employee system?\n");

    loop {
        print!("Y/n: ");
        io::stdout().flush().expect("Failed to empty stdout buffer");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "y" | "Y" => return true,
                "n" | "N" => return false,
                _ => println!("Invalid input. Please enter 'Y' or 'n'."),
            },
            Err(_) => println!("Error reading input. Try again"),
        }
    }
}

fn postrun(db: &mut HashMap<String, Vec<String>>) {
    println!("\nPress any key to continue...");
    
    let mut buffer = [0; 1];

    io::stdin().read_exact(&mut buffer).expect("Failed to read from stdin");

    // Clear the newline character from the buffer
    io::stdin().read_line(&mut String::new()).expect("Failed to clear buffer");

    // Optionally, you can print a newline to separate the prompt from the user's input
    println!();
    interface(db);
}

fn interface(db: &mut HashMap<String, Vec<String>>) {
    println!("\nEmployee Interface");
    println!("Options:");
    println!("1. List all employees");
    println!("2. List employees by department");
    println!("3. Add an employee");
    println!("4. Exit");

    let choice: i8 = loop {
        print!("Choice: ");
        io::stdout().flush().expect("Failed to empty stdout buffer");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse() {
                Ok(val) if (1..=4).contains(&val) => break val,
                _ => println!("Invalid input. Try again"),
            },
            Err(_) => println!("Error reading input. Try again"),
        }
    };

    match choice {
        1 => list_all(db),
        2 => list_by_dept(db),
        3 => add(db),
        4 => exit(0),
        _ => unreachable!(),
    }
}

fn list_all(db: & mut HashMap<String, Vec<String>>) {
    let mut names: Vec<String> = db.values().flat_map(|v| v.clone()).collect();
    names.sort();
    for name in &names {
        println!("{}", name);
    }
    postrun(db);
}

fn list_by_dept(db: &mut HashMap<String, Vec<String>>) {
    let mut depts: Vec<String> = db.keys().cloned().collect();
    depts.sort();

    println!("Which department's employees would you like to see?");
    for (i, dept) in (1..).zip(&depts) {
        println!("{i}. {dept}");
    }

    let choice: usize = loop {
        print!("Choice: ");
        io::stdout().flush().expect("Failed to empty stdout buffer");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse() {
                Ok(val) if val > 0 && val <= depts.len() => break val,
                _ => println!("Invalid input. Try again"),
            },
            Err(_) => println!("Error reading input. Try again"),
        }
    };

    let chosen_dept = &depts[choice - 1];
    println!("Employees in {} department:", chosen_dept);

    if let Some(names) = db.get(chosen_dept) {
        let mut sorted_names = names.clone();
        sorted_names.sort();
        for name in &sorted_names {
            println!("{}", name);
        }
        postrun(db);
    } else {
        println!("No employees found for the selected department.");
        postrun(db);
    }
}

fn add(db: &mut HashMap<String, Vec<String>>) {
    let depts: Vec<String> = db.keys().cloned().collect();
    let mut sorted_depts = depts.clone();
    sorted_depts.sort();

    println!("Current departments:");
    for (i, dept) in (1..).zip(&sorted_depts) {
        println!("{i}. {dept}");
    }

    let dept: String = loop {
        println!("To which department?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(s) => break s,
            _ => println!("Invalid department. Try again."),
        }
    };

    let employees = db.entry(dept.clone()).or_default();

    let employee: String = loop {
        println!("Enter the name of the employee");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if !input.is_empty() {
            break input.to_string();
        } else {
            println!("Name cannot be empty. Try again.");
        }
    };

    employees.push(employee);
    println!("Company departments: {:?}", db);
    println!();
    postrun(db)
}
