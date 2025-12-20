mod tokenizer;
use tokenizer::{tokenize, Token};
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let cmd = command.trim();
        let tokens = tokenize(cmd).unwrap();

        if tokens[0]==Token::Word(String::from("echo")){
            let msg = &tokens[1..];
            for t in msg {
                print!("{t} ");
            }
            println!();
        }
        else {
            match cmd {
                "" => continue,
                "exit" => return,
                _ => println!("{cmd}: command not found"),
            }
        }
    }
}
