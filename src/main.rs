#[allow(unused_imports)] 
mod tokenizer;
use tokenizer::{tokenize, Token};

mod type_command;
use type_command::{type_command_call_this_in_match, BUILTINS};
mod external_commands;
use external_commands::{externalcommand};
mod pipe;
use pipe::{split_by_pipe,execute_pipeline};

use std::io::{self, Write};
use std::process::Command;
use std::env;


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();  //to display output directly in terminal instead of storing in memory and waiting

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap(); //handle unwrap better here

        let cmd = command.trim();
        let tokens = tokenize(cmd).unwrap();
        if tokens.is_empty() {continue;}  //to handle enter key
        //used for running external command
        let query= match &tokens[0] {
            Token::Word(ms) => ms.as_str(),
            Token::Pipe => {panic!("check type_command file, why is token a pipe");} 
        };

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
                    "echo" =>{  //this why Token enum is defined
                        let msg = &tokens[1..];
                        for t in msg {
                            print!("{t} ");
                        }
                        println!();
                    }
                    _ =>if let Some(path) = type_command::find_in_path(&query){
                            match external_commands::externalcommand (&tokens) {
                                Ok(code) => {},
                                Err(e) => eprintln!("error: {}", e),
                            }
                        }
                        else{println!("{}: command not found", tokens[0]);},
                }
            
            }
            Token::Pipe => {
                    println!("syntax error near '|'"); //idk what to do , modify this
                }
            _ => {println!("other token than Word or PIPE, modifyy the tokenise.rs to accomodate uit.")} 
        }        
    }
}




