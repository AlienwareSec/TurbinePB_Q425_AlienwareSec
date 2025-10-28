use std::io::{self, Write};

fn main() {
    println!("=========================");
    println!("     Counter Game");
    println!("=========================\n");

    let mut counter = 0;

    println!("Starting Value → {}", counter);
    println!("Type 'up' to increase, 'down' to decrease, or 'exit' to quit.\n");

    loop {
        print!("Your move: ");
        io::stdout().flush().unwrap(); 

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input.");

        match command.trim().to_lowercase().as_str() {
            "up" => {
                counter += 1;
                println!("Increased! Counter is now {}\n", counter);
            }
            "down" => {
                counter -= 1;
                println!("Decreased! Counter is now {}\n", counter);
            }
            "exit" => {
                println!("\nExiting the Counter Game. Final value: {}", counter);
                break;
            }
            _ => println!("Invalid command! Try 'up', 'down', or 'exit'.\n"),
        }
    }
}
