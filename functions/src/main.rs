fn main() {
    println!("Hello, world!");

    another_function(5);
    another_function(plus_one(2));
}

fn another_function(x: i32) {
    println!("This is the value of x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
