use std::io::{self, Write};
use std::env;
use std::path::Path;

use crate::tokenizer::{Token, tokenize};

pub const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

pub fn type_command_call_this_in_match(tokens:&Vec<Token>){

   
                            let query= match &tokens[1] {
                                Token::Word(ms) => ms.as_str(),
                                Token::Pipe => {panic!("check type_command file, why is token0 a pipe");}
                            };
                            if BUILTINS.contains(&query) {
                                println!("{query} is a shell builtin");
                            } else if let Some(path) = find_in_path(query) {
                                println!("{query} is {path}");
                            } else {
                                println!("{query} not found");
                            }
}

pub fn find_in_path(cmd: &str) -> Option<String> {
    let path_var = env::var("PATH").ok()?;

    for dir in path_var.split(':') {
        let full_path = format!("{}/{}", dir, cmd);
        if Path::new(&full_path).exists() {
            return Some(full_path);
        }
    }
    None
}
