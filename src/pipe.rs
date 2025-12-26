#[allow(unused_imports)] 

use crate::Token;

use nix::unistd::{Pid,pipe, execvp,fork, ForkResult};
use nix::sys::wait::{waitpid};
use std::ffi::CString;
use nix::unistd::dup2;

use std::os::fd::{FromRawFd, OwnedFd};


pub fn split_by_pipe(tokens: &[Token]) -> Vec<Vec<Token>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    for token in tokens {
        match token {
            Token::Pipe => {
                result.push(current);
                current = Vec::new();
            }
            _ => current.push(token.clone()),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}


pub fn execute_pipeline(commands: Vec<Vec<String>>) {
    let mut children: Vec<Pid> = Vec::new();  
    let mut prev_read: Option<OwnedFd> = None;

    for (i, cmd) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;

        let pipefds = if !is_last {
            Some(pipe().unwrap())
        } else {
            None
        };

        match unsafe { fork() }.unwrap() {
            ForkResult::Child => {
                // If there was a previous pipe, read from it (dup old -> stdin)
               if let Some(ref read_fd) = prev_read {
                    let mut stdin = unsafe { OwnedFd::from_raw_fd(0) };
                    dup2(&read_fd, &mut stdin).unwrap(); //this is the new and correct method to use dup2. DONT trust chatgpt here.
                    std::mem::forget(stdin); // Prevent closing stdin
               }
                // If not last command, write into pipe (dup write_fd -> stdout)
                if let Some((_, write_fd)) = &pipefds {
                    let mut stdout = unsafe { OwnedFd::from_raw_fd(1) };
                    dup2(write_fd, &mut stdout).unwrap();
                    std::mem::forget(stdout); // Prevent closing stdout
                }
                 // Drop unused FDs (auto-close)
                drop(prev_read);
                drop(pipefds);

      // Exec
                let cstrs: Vec<CString> =
                    cmd.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
                let args: Vec<&CString> = cstrs.iter().collect();

                // execvp never returns on success, only on error
                if let Err(_) = execvp(&args[0], &args) {
                    std::process::exit(1);
                }
                // This point is unreachable but needed for type checking
                unreachable!()
            }

            ForkResult::Parent { child } => {
                children.push(child);
                // Parent no longer needs previous read end
                drop(prev_read.take());

                // Parent keeps read end for next command
                if let Some((read_fd, write_fd)) = pipefds {
                    drop(write_fd);      // close write end
                    prev_read = Some(read_fd);
                }
            }

        }
    }

    // wait for all children explicitly
    for pid in children {
        waitpid(pid, None).unwrap();
    }
}

