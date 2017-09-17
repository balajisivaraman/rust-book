#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, another: &Rectangle) -> bool {
        //potentially incorrect implementation here
        self.area() > another.area()
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };
    // fn defined as not taking self as the first argument
    // this behaviour is similar to static methods
    let square = Rectangle::square(10);
    println!("the area of rectangle {:#?} is: {}", rect1, rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.length
// }
