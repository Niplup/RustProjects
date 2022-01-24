#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //Can also do multiple impl blocks and put functions/methods in each of them
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn hold_within(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 60,
    };

    let rect4 = Rectangle::square(20);

    if rect1.width() {
        println!("The rectangle has a non-zero width: {}", rect1.width());
    }
    println!("Can rect1 hold rect2 within: {}", rect1.hold_within(&rect2));
    println!("Can rect1 hold rect3 within: {}", rect1.hold_within(&rect3));

    println!("The area of the rectangle is {}", rect1.area());
    println!("The area of the square is {}", rect4.area())

}
