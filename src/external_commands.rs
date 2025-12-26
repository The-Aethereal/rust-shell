#[allow(unused_imports)] 
use crate::Token;
use crate::pipe;
use std::process::Command;


//helper function for converting Vec<Vec<Token>> to Vec<Vec<String>> in pipe command
pub fn tokens_to_argv(tokens: &[Token]) -> Vec<String> {
    tokens.iter().filter_map(|t| {
        match t {
            Token::Word(s) => Some(s.clone()),
            _ => None,
        }
    }).collect()
}

pub fn externalcommand(tokens: &Vec<Token>) -> std::io::Result<i32> {
    let has_pipe = tokens.iter().any(|t| matches!(t, Token::Pipe));     // Check if pipeline exists


    if has_pipe {
        // Split tokens into pipeline segments
        let segments = pipe::split_by_pipe(tokens);

        // Convert each segment into argv (Vec<String>)
        let commands: Vec<Vec<String>> = segments
            .into_iter()
            .map(|segment| tokens_to_argv(&segment))
            .filter(|argv| !argv.is_empty())
            .collect();

        // Execute pipeline (handles fork/exec internally)
        pipe::execute_pipeline(commands);

        // Pipelines usually return status of last command
        return Ok(0); // placeholder for now
    }
    else {
        let argv = tokens_to_argv(tokens);

        if argv.is_empty() {
            return Ok(0);
        }

        let mut cmd = Command::new(&argv[0]);

        if argv.len() > 1 {
            cmd.args(&argv[1..]);
        }

        let status = cmd.spawn()?.wait()?;
        Ok(status.code().unwrap_or_default())
    }
}
