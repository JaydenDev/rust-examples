// JaydenDev
// Basic rust string to binary converter
// MIT License
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let string = input.to_string();
    let mut string_in_binary = "".to_string();
    for character in string.clone().into_bytes() {
        string_in_binary += &format!("0{:b} ", character);
    }
   println!("\"{}\" in binary is {}", string, string_in_binary);
}