use std::fs;
use std::io::Write;
use std::process::Command;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string();
}

fn help() {
    println!("--del          ;deletes file with given path;\n");
    println!("--find         ;opens file location in finder;\n");
}

fn del(f_path: &str) -> bool {
    match fs::remove_file(f_path) {
        Ok(_) => true,
        Err(e) => false
    }
}

fn find(mut f_dir: &str) -> bool {
    println!("accessing path...\n");
    if f_dir == "" { //if the user doesn't type in anything after calling the function
        f_dir = "."; //default arg - the current directory
    }
    match Command::new("explorer")
        .arg(format!("\"{}\"", f_dir)) 
        .spawn()   // spawning the process
    {
        Ok(_) => {
            true
        },
        Err(e) => {
            eprintln!("Error launching given directory: {}",e);
            false
        }
    }
}

fn main() {
    loop {
        let input = prompt("alice> ");
        let words: Vec<&str> = input.split_whitespace().collect();
        if let Some(command) = words.get(0) {
            match *command {
                "help" => help(),
                "del" => {
                    if let Some(extra) = words.get(2) {
                        println!("incorrect usage: extra word {}", extra)
                    } else {
                        if let Some(file_path) = words.get(1) {
                            if del(file_path) {
                                println!("File deleted\n");
                            } else {
                                eprintln!("Error deleting the file\n");
                            }
                        } else {
                            println!("Usage: del <file_path>\n");
                        }
                    }                   
                }
                "find" => {
                    if let Some(extra) = words.get(2) {
                        println!("incorrect usage: extra word {}", extra)
                    } else {
                        if let Some(f_dir) = words.get(1) {
                            if find(f_dir) {
                                println!("File Explorer successfully launched.");
                            } else {
                                eprintln!("Error launching File Explorer.");
                            }
                        }
                    }
                    
                    
                    
                }
                
                _ => println!("Unknown command: {}\n", command),
            }
        }
    }
}

