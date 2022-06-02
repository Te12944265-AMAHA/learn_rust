struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    
    let rect2 = Rectangle::square(20);
    println!("rect 1 can hold rect 2? {}", rect1.can_hold(&rect2));
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
