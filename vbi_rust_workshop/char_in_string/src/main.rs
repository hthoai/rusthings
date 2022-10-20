use std::io;

fn main() {
    let input = "adbcdaDd";
    let mut not_found_chars = String::new();
    let mut user_input = String::new();
    let mut count = 0;
    println!("Please type one character:");
    io::stdin().read_line(&mut user_input).unwrap();
    
    for char in input.chars() {
        if char == user_input.as_bytes()[0] as char {
            count += 1;
        } else {
            not_found_chars.push(char);
        }
    }

    println!("{}, \"{}\"", count, not_found_chars);
}
