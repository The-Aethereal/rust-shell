#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
         print!("$ ");
    io::stdout().flush().unwrap();
    let mut t: String= String::new();
    io::stdin().read_line(&mut t).unwrap();
    println!("{}: command not found", t.trim())
    }
}
