fn main() {
    let mut x = 5;
    println!("The number is {}", x);
    x = 6;
    {
        let x = x * 2;
        println!("The number is {}", x);

    }
    println!("The number is {}", x);

}
