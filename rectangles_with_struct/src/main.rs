#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width>=other.width && self.height>=other.height)
        || (self.width>=other.height && self.height>=other.width)
    }
}

fn main() {
    let rect = Rectangle { width: 4, height: 8 };
    dbg!(&rect);
    println!("Rectangle's area: {} pixel²", rect.area());

    let rect1 = Rectangle { width: 8, height: 4 };
    let rect2 = Rectangle { width: 10, height: 20 };
    let rect3 = Rectangle::new(2, 20);
    let square = Rectangle::new_square(42);
    println!("{} in {}: {}", stringify!(rect), stringify!(rect1), rect1.can_hold(&rect));
    println!("{} in {}: {}", stringify!(rect), stringify!(rect2), rect2.can_hold(&rect));
    println!("{} in {}: {}", stringify!(rect), stringify!(rect3), rect3.can_hold(&rect));
    println!("{} in {}: {}", stringify!(rect), stringify!(square), square.can_hold(&rect));
}