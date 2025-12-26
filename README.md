A Unix-like Shell in Rust

1. [Project Goals](#-project-goals)
2. [Architecture Overview](#-architecture-overview)
3. [Build & Run](#-build--run)
4. [Future Work](#-future-work)

---

## Project Goals

- Learn **how Unix shells work internally**
- Understand **process lifecycles, fork/exec, pipes,redirection and file descriptors**
- Use Rust to safely model low-level OS behavior
- Learning about I/O, error handling, and syscalls
---
##  Architecture Overview

REPL
 ↓
Lexer
 ↓
Parser
 ↓
Executor
 ├── Builtins
 ├── External Commands
 ├── Pipelines(fork + exec + dup2)
 ├── Redirection
 └── Job Control


---


## Build & Run

```bash
git clone https://github.com/The-Aethereal/rust-shell.git
cargo build
cargo run
```
ignore warnings :)

## Future Work
-I/O redirection (>, <)
-Background jobs (&)
-Job control and process groups
-Replacing std::process::Command with pure fork + exec
-Auto Tab completion
