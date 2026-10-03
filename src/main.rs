use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // Each task is (name, is_done)
    let mut tasks: Vec<(String, bool)> = Vec::new();

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Complete task");
        println!("4. Quit");

        match read_line().as_str() {
            "1" => {
                println!("Task name:");
                let name = read_line();
                tasks.push((name, false));
                println!("Added!");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks yet.");
                }
                for (i, (name, done)) in tasks.iter().enumerate() {
                    let mark = if *done { "x" } else { " " };
                    println!("{}. [{}] {}", i + 1, mark, name);
                }
            }
            "3" => {
                println!("Task number:");
                match read_line().parse::<usize>() {
                    Ok(n) if n >= 1 && n <= tasks.len() => {
                        tasks[n - 1].1 = true;
                        println!("Marked as done!");
                    }
                    _ => println!("Invalid task number."),
                }
            }
            "4" => {
                println!("Bye!");
                break;
            }
            _ => println!("Please pick 1-4."),
        }
    }
}