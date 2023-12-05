use std::fs;
use std::io::Write;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}

fn help() {
    println!("--del          ;deletes file with given path;\n");
    println!("")
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
                _ => println!("Unknown command: {}\n", command),
            }
        }
    }
}

