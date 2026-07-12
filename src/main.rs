use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
       
        
        let mut line = String::new();
        
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        let (command, text) = match line.split_once(' ') {
            Some((command, text)) => (command, text),
            None => (line, ""),
        };

        if command == "type" {
            if text == "echo" || text == "exit" || text == "type" {
                println!("{} is a shell builtin", text)
            } else {
                match find_exe(text)
                {
                    Some(path) => println!("{} is {}", text, path.display()),
                    None => println!("{}: not found", text)
                }
            };
        } else if command == "echo" {
            println!("{}", text)
        } else if command == "exit" {
            return;
        } else {
            println!("{}: command not found", command)
        }
    }
}

fn find_exe(cmd : & str) -> Option<PathBuf>{

    let path_var = env::var("PATH").ok()?;
    for dir in env::split_paths(&path_var)
    {
        let candidate = dir.join(cmd);
        if candidate.is_file(){
            if let Ok(metadata) = std::fs::metadata(&candidate){
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(candidate)
                }
            }
        }
    }
    None
}