mod tokenizer;
use tokenizer::{tokenize, Token};

mod type_command;
use type_command::{type_command_call_this_in_match, BUILTINS};

use std::io::{self, Write};


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let cmd = command.trim();
        let tokens = tokenize(cmd).unwrap();
        if(tokens.is_empty()){continue;}

        match &tokens[0] {
            Token::Word(first_token_here) => {
                match first_token_here.as_str() {
                    "exit" => {return},
                    "type" => { if tokens.len() < 2 {
                                    println!("type: missing argument");
                                    continue;
                                } 
                                else { type_command_call_this_in_match(&tokens);}
                    }
                    "echo" =>{
                        let msg = &tokens[1..];
                        for t in msg {
                            print!("{t} ");
                        }
                        println!();
                    }
                    _ => println!("{}: command not found", tokens[0]),
                }
            
            }
            Token::Pipe => {
                    println!("syntax error near '|'");
                }
            _ => {println!("other token than Word or PIPE, modifyy the tokenise.rs to accomodate uit.")} 
        }        
    }
}
