use std::fs;
use std::io::Write;
use std::path::Path;
mod fetch_hardware;
use fetch_hardware::fetch_hardware;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}

fn help() {
    println!("--del          ;deletes file with given path;\n");
    println!("--rename-item  ;renames item;\n");
    println!("--sys-info     ;view hardware info;\n");
}

fn rename_item(old_file_path: &str, new_file_path: &str) -> bool {
    let old_path = Path::new(old_file_path);
    let new_path = old_path.with_file_name(new_file_path);
    match fs::rename(old_file_path, new_path) {
        Ok(_) => {
            println!("file renamed");
            true
        },
        Err(e) => {
            eprintln!("error renaming file: {}", e);
            false
        },
    }
}

fn del(f_path: &str) -> bool {
    match fs::remove_file(f_path) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("error: {}", err);
            false
        }
    }
}

fn main() {
    loop {
        let input: String = prompt("alice> ");
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
                "rename-item" => {
                    // Check if there are exactly three words in the command
                    if words.len() != 3 {
                        println!("Usage: rename-item <old_file_path> <new_file_name>\n");
                    } else {
                        let old_file_path = words.get(1).unwrap_or(&"").to_string();
                        let new_file_path = words.get(2).unwrap_or(&"").to_string();
                        if rename_item(&old_file_path, &new_file_path) {
                            println!("File successfully renamed\n");
                        } else {
                            eprintln!("Error renaming the file\n");
                        }
                    }
                }
                "sys-info" => {
                    fetch_hardware()
                }
                _ => println!("Unknown command: {}\n", command),
            }
        }
    }
}

