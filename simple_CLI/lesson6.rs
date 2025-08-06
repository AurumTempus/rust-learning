use std::io::{self, BufRead};

enum Command {
    Add(i32, i32),
    Substract(i32, i32),
    Echo(String),
    Invalid,
}

fn main() {
    let commands = vec![
        Command::Add(5, 3),
        Command::Substract(10, 123),
        Command::Echo("Hello, man".to_string()),
        Command::Invalid,
    ];

    for cmd in commands {
        println!("{}", process(cmd));
    }

    println!("Enter a command (add x y, substract x y, echo text, or invalid)");
    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = line.unwrap().trim().to_string();
        if input.is_empty() {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = match parts[0] {
            "add" if parts.len() == 3 => {
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Add(a, b)
            }
            "substract" if parts.len() == 3 => {
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Substract(a, b)
            }
            "echo" if parts.len() >= 2 => {
                let msg = parts[1..].join(" ");
                Command::Echo(msg)
            }
            "invalid" => Command::Invalid,
            _ => Command::Invalid,
        };

        println!("{}", process(cmd));
    }
}

fn process(command: Command) -> String {
    match command {
        Command::Add(a, b) => format!("Sum: {}", a + b),
        Command::Substract(a, b) => format!("Substract: {}", a - b),
        Command::Echo(msg) => format!("Echo: {msg}"),
        Command::Invalid => "Invalid format".to_string(),
    }
}
