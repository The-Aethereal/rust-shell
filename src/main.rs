use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let cmd = command.trim();

        if let Some(msg) = cmd.strip_prefix("echo ") {
            println!("{msg}");
        } else {
            match cmd {
                "" => continue,
                "exit" => return,
                _ => println!("{cmd}: command not found"),
            }
        }
    }
}
