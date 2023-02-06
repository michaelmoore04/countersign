use std::env; 
use rand::{Rng, thread_rng};

const DEFAULT_LENGTH: usize = 32;

fn get_random_char(charset: &[u8]) -> char {
    let idx = thread_rng().gen_range(0..charset.len());
    charset[idx] as char
}

fn main() {
    let chars: String = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "0123456789",
        "!@#$%&*").to_string();
    let charset: &[u8] = &chars.into_bytes();

    let length: usize;
    let args: Vec<String> = env::args().collect();

    length = match args.len() {
        2 => match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => DEFAULT_LENGTH
        },
        _ => DEFAULT_LENGTH
    };

    let pass: String = (0..length)
        .map(|_| get_random_char(&charset))
        .collect();
    println!("{}", pass);
}
