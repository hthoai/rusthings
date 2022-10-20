use std::io;


fn char_in_string(input: &str, user_input: &str) {
    let mut not_found_chars = String::new();
    let mut count = 0;

    for char in input.chars() {
        if char == user_input.as_bytes()[0] as char {
            count += 1;
        } else {
            not_found_chars.push(char);
        }
    }

    println!("{}, \"{}\"", count, not_found_chars);
}


fn main() {
    let input = "adbcdaDd";
    let mut user_input = String::new();
    println!("Please type one character:");
    io::stdin().read_line(&mut user_input).unwrap();
    char_in_string(input, &user_input);
}
