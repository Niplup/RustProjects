use std::io;

fn main() {
    println!("Enter your input: ");
    println!("The output is: {}", nth_fibonacci(string_input_to_int()));
}

fn string_input_to_int() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => std::process::exit(1),
    };
    n
}

fn nth_fibonacci(n: u32) -> u32 {
    if n <= 1 {return n;}
    nth_fibonacci(n-1) + nth_fibonacci(n-2)
}
