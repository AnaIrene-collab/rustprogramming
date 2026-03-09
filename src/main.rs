
use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            Command::new("ls")
                .arg(dir)
                .status()
                .expect("Failed to execute ls");
        }

        FileOperation::Display(file) => {
            Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");
        }

        FileOperation::Remove(file) => {
            Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn main() {
    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter directory path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::List(path.trim().to_string()));
            }

            "2" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Display(path.trim().to_string()));
            }

            "3" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                println!("Enter content:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    path.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Remove(path.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option"),
        }
    }
}