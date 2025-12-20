#[allow(unused_imports)] 
use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};
use std::fs;


use crate::tokenizer::{Token, tokenize};

// add more builtin commands here
pub const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

pub fn type_command_call_this_in_match(tokens:&Vec<Token>){

   
                            let query= match &tokens[1] {
                                Token::Word(ms) => ms.as_str(),
                                Token::Pipe => {panic!("check type_command file, why is token a pipe");} //idk why i wrote this ! fix if you understand
                            };
                            if BUILTINS.contains(&query) {
                                println!("{query} is a shell builtin"); // check if you could do any better
                            } else if let Some(path) = find_in_path(query) {  
                                println!("{query} is {}", path.to_string_lossy()); 
                            } else {
                                println!("{query} not found");
                            }
}

//mindblowing function ahead
pub fn find_in_path(cmd: &str) -> Option<PathBuf> {
    if cmd.contains('/') {                       //  If cmd contains '/', don't search PATH
        let path = PathBuf::from(cmd);
        if is_executable(&path) {
            return Some(path);
        } else {
            return None;
        }
    }

    let path_var = env::var("PATH").ok()?;

    for dir in path_var.split(':') {
        let dir = if dir.is_empty() { "." } else { dir };

        let full_path = Path::new(dir).join(cmd);

        if is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

//go through following once more
fn is_executable(path: &Path) -> bool {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !meta.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    {
        true
    }
}
