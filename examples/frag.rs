
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn main() {
    let width = 30;
    let height = 50;
    let rect = Rectangle { width, height }; // comment

    println!("The rect is {:#?}.", rect);
    println!("The area is {}.", area(&rect));
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.area()
}
