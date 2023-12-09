use std::fs;
use std::fs::File;
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
    println!("--create       ;creates new file in Desktop;\n")
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
        .arg(&f_dir)
        .spawn()   // spawning the process
    {
        Ok(_) => true,

        Err(e) => {
            false
        }
    }
}

fn create(f_name: &str) -> bool {
    match File::create(f_name) {
        Ok(_) => true,
        
        Err(e) => {
            false
        }
    }
}

fn main() {
    loop {
        let input = prompt("alice> ");
        // find User/OneDrive Bellarmine/
        let words: Vec<&str> = input.split_whitespace().collect();
        if let Some(command) = words.get(0) {
            match *command {
                "help" => help(),
                "del" => {
                    if let Some(extra) = words.get(2) {
                        eprintln!("incorrect usage: extra word {}", extra)
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
                   if words.len() >= 2{
                        let fixed_path = words[1..].join(" ");
                        if find(&fixed_path) {
                            println!("File explorer succesfully launched\n");
                        } else {
                            eprintln!("Error opening file explorer\n");
                        }
                   } else {
                        eprintln!("Usage: find <directory_path>\n");
                   }     
                }

                "create" => {
                    if let Some(extra) = words.get(2) {
                        eprintln!("incorrect usage: extra word {}", extra)
                    } else {
                        if let Some(f_name) = words.get(1) {
                            if create(f_name) {
                                println!("File successfully created\n");
                            } else {
                                eprintln!("Error creating file\n"); 
                            }
                        }
                    }
                }
                _ => println!("Unknown command: {}\n", command),
            }
        }
    }
}