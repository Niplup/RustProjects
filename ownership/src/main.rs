fn main() {
    let mut d = String::from("hello");
    d.push_str(", world");
    println!("{}", d);

    let s = 2;
    let x = tuple_return(&mut d, s);
    println!("The string is {} and the int is {}", x.0, x.1);
}

fn tuple_return(d: &mut String, s: u32) -> (&mut String, u32) {
    d.push_str("foo");
    (d,s+5)
}
