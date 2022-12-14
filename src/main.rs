use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    // accepting multiple commands
    print!("Welcome to the shell! (type 'exit' to exit)\n");

    loop {
        let mut input = String::new();

        print!("> ");
        stdout().flush();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap(); // trim() removes the trailing newline
        let args = parts;

        // cd command as built in command
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("cd: {}", e);
                }
            }
            "exit" => {
                break;
            }
            command => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    }
                    Err(e) => {
                        eprintln!("{}: command not found", command);
                    }
                }
            }
        }
    }
}
