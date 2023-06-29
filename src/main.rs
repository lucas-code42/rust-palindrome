use std::io;

fn user_input() -> String {
    let mut i: String = String::new();
    match io::stdin().read_line(&mut i) {
        Ok(_) => {
            return i;
        }
        Err(error) => println!("error: {}", error),
    }

    return String::from("");
}

fn is_palindrome(data: String) -> bool {
    let trimmed_data: String = data.trim().to_string();
    let reversed_data: String = trimmed_data.chars().rev().collect();

    if trimmed_data == reversed_data {
        println!("is a palindrome!");
        true
    } else {
        println!("Não é um palindrome!");
        false
    }
}

fn main() {
    let x: String = user_input();
    println!("{}", is_palindrome(x));
}
