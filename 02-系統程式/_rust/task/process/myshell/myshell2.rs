use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("myshell > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => break,
            "cd" => {
                let new_dir = args.get(0).cloned().unwrap_or("/");
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("myshell: cd: {}", e);
                }
            }
            _ => {
                match Command::new(command).args(&args).spawn() {
                    Ok(mut child) => {
                        child.wait().expect("子行程執行出錯");
                    }
                    Err(_) => {
                        eprintln!("myshell: 指令未找到: {}", command);
                    }
                }
            }
        }
    }
}