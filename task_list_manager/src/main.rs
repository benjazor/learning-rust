use std::fs;
use std::io;
use std::{env, process};

/* This cli tool helps manage task lists and saving them for later. */
struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: &str) -> Task {
        Task {
            name: name.to_string(),
            completed: false,
        }
    }

    fn toggle(&mut self) -> bool {
        self.completed = !self.completed;
        !self.completed
    }

    fn to_string(&self) -> String {
        let mut result = String::from("");
        if self.completed {
            result.push('X');
        } else {
            result.push('O');
        }
        result.push('\t');
        result.push_str(self.name.as_str());
        result
    }

    fn from(data: &str) -> Task {
        if data.len() < 3 {
            return Task {
                name: "".to_string(),
                completed: false,
            };
        }
        Task {
            name: String::from(&data[2..]),
            completed: data[..1].eq("X"),
        }
    }
}

fn main() {
    // Check if we have at lease one argument
    if env::args().len() < 2 {
        eprintln!("This command requires at least 1 argument, try \"help\".");
        process::exit(1);
    }

    match env::args().nth(1).unwrap().as_str() {
        // Task list
        "display" => {
            // Check if we have at least 2 arguments
            if env::args().len() < 3 {
                eprintln!("This command requires at least 2 argument, try \"help\".");
                process::exit(1);
            }
            let content = fs::read_to_string(env::args().nth(2).unwrap()).unwrap();
            // Display the content of the file (formatted)
            println!("DONE\tTASK");
            for line in content.lines() {
                println!("{}", line);
            }
        }
        "create" => {
            // Check if we have at lest 2 arguments
            if env::args().len() < 3 {
                eprintln!("This command requires at least 2 argument, try \"help\".");
                process::exit(1);
            }
            fs::write(env::args().nth(2).unwrap(), "O\tsample task\n")
                .expect("Couldn't create the file.");
            println!("New task list \"{}\" created!", env::args().nth(2).unwrap());
        }
        "remove" => {
            // Check if we have at lest 2 arguments
            if env::args().len() < 3 {
                eprintln!("This command requires at least 2 argument, try \"help\".");
                process::exit(1);
            }

            println!(
                "Please confirm the removal of \"{}\" by typing \"yes\"",
                std::env::args().nth(2).unwrap()
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");

            match input.as_str().trim() {
                "y" | "yes" => {
                    fs::remove_file(env::args().nth(2).unwrap())
                        .expect("Cloud not remove the file");
                    println!("Task list \"{}\" removed!", env::args().nth(2).unwrap());
                }
                _ => {
                    println!(
                        "Removal of \"{}\" was canceled.",
                        env::args().nth(2).unwrap()
                    )
                }
            }
        }

        // Task
        "add" => {
            // Check if we have at least 3 arguments
            if env::args().len() < 4 {
                eprintln!("This command requires at least 3 argument, try \"help\".");
                process::exit(1);
            }
            let task = Task::new(env::args().nth(3).unwrap().as_str());
            let mut content =
                fs::read_to_string(env::args().nth(2).unwrap()).expect("Could not read the file.");
            content.push_str(task.to_string().as_str());
            content.push('\n');
            fs::write(env::args().nth(2).unwrap(), content).expect("Could not save the new task");
            println!(
                "Successfully added \"{}\" to {}!",
                env::args().nth(3).unwrap(),
                env::args().nth(2).unwrap()
            )
        }

        "delete" => {
            // Check if we have at least 3 arguments
            if env::args().len() < 4 {
                eprintln!("This command requires at least 3 argument, try \"help\".");
                process::exit(1);
            }
            let content =
                fs::read_to_string(env::args().nth(2).unwrap()).expect("Could not read the file.");

            let mut new_content = String::from("");
            for line in content.lines() {
                let task = Task::from(line);
                if line.len() > 1 && !task.name.eq(&env::args().nth(3).unwrap()) {
                    println!("{}", &line);
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }

            fs::write(env::args().nth(2).unwrap(), &new_content)
                .expect("Could not delete the task");
        }

        "toggle" => {
            // Check if we have at least 3 arguments
            if env::args().len() < 4 {
                eprintln!("This command requires at least 3 argument, try \"help\".");
                process::exit(1);
            }
            let content =
                fs::read_to_string(env::args().nth(2).unwrap()).expect("Could not read the file.");

            let mut new_content = String::from("");
            for line in content.lines() {
                let mut task = Task::from(line);
                if task.name.eq(&env::args().nth(3).unwrap()) {
                    task.toggle();
                    new_content.push_str(task.to_string().as_str());
                } else {
                    new_content.push_str(line);
                }
                new_content.push('\n');
            }

            fs::write(env::args().nth(2).unwrap(), &new_content)
                .expect("Could not delete the task");
        }

        // Other
        "help" => println!("{}", fs::read_to_string("help.txt").unwrap()),
        "test" => {
            let mut my_task = Task::new("Faire les courses");
            let task_2 = Task::from(&my_task.to_string().as_str());
            my_task.toggle();
            println!("{}", &my_task.to_string());
            println!("{}", &task_2.to_string());
        }
        _ => {
            eprintln!(
                "Unknown argument \"{}\", try \"help\".",
                env::args().nth(1).unwrap()
            );
            process::exit(1);
        }
    }
}
