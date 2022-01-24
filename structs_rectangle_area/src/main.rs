use std::io;
use std::io::Write;
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let (w, h) = parser();
    let rect1 = Rectangle {
        width: w,
        height: h,
    };

    println!("The dimensions (debug) is: {:?}", rect1);
    println!("The area (nostruct) is: {}", rect_area_no_struct((w, h)));
    println!("The area (struct) is: {}", rect_area_struct(&rect1));
}

fn parser() -> (u32, u32) {
    let (mut width, mut height) = (String::new(), String::new());

    print!("Enter your width: ");
    io::stdout().flush().expect("Unable to flush");
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read input");

    print!("Enter your height: ");
    io::stdout().flush().expect("Unable to flush");
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read input");

    let width: u32 = width
        .trim()
        .parse()
        .expect("Invalid input");
    let height: u32 = height 
        .trim()
        .parse()
        .expect("Invalid input");

    println!("W: {} H: {}", width, height);
    (width, height)
}

fn rect_area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn rect_area_no_struct(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
