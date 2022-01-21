use std::io;

fn main() {
    println!("The temperature is {:.2}", fahrenheit_to_celsius());
}

fn fahrenheit_to_celsius() -> f64 {
    println!("Enter which units you are starting in (F or C)");

    let mut choice = String::new();
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();
    println!("Enter your temperature");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => std::process::exit(1),
    };

    if choice == "f" || choice == "F" {
        (temp - 32.0) * 0.556
    } else if choice == "c" || choice == "C" {
        temp * 1.8 + 32.0
    } else {
        std::process::exit(1)
    }
}
