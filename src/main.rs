use std::env; 
use rand::{Rng, thread_rng};

const DEFAULT_LENGTH: usize = 32;

fn get_random_char(charset: &[u8]) -> char {
    let idx = thread_rng().gen_range(0..charset.len());
    charset[idx] as char
}

fn main() {
    let uppercase_chars: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase_chars: &str = "abcdefghijklmnopqrstuvwxyz";
    let numeric_chars: &str = "0123456789";
    let special_chars: &str = "!@#$%&*";
    
    let mut length: usize = DEFAULT_LENGTH;
    let mut alpha: bool = false;
    let mut numeric: bool = false;
    let mut special: bool = false;
    let args: Vec<String> = env::args().collect();
    
    // length = match args.len() {
    //     2 => match args[1].trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => DEFAULT_LENGTH
    //     },
    //     _ => DEFAULT_LENGTH
    // };

    let mut i = 1;

    while i < args.len() {
        if (args[i] == "-l" || args[i] == "--length") {
            length = match args[i + 1].trim().parse() {
                Ok(num) => num,
                Err(_) => DEFAULT_LENGTH
            };
            i += 1;
        } else if (args[i] == "-a" || args[i] == "--alpha") {
            alpha = true
        } else if (args[i] == "-n" || args[i] == "--numeric") {
            numeric = true
        } else if (args[i] == "-s" || args[i] == "--special") {
            special = true
        }
        i += 1;
    }

    let mut chars: String = "".to_owned();
    if alpha {
        chars.push_str(uppercase_chars);
        chars.push_str(lowercase_chars);
    }
    if numeric {
        chars.push_str(numeric_chars);
    }
    if special {
        chars.push_str(special_chars);
    }
    let charset: &[u8] = &chars.into_bytes();

    let pass: String = (0..length)
        .map(|_| get_random_char(&charset))
        .collect();
    println!("{}", pass);
}
