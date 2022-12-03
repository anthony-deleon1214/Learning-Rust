// Implementing a rectangle struct with width and height fields
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing the area function as a method on the Rectangle struct
// Functions within impl block are all associated with a struct
// If the first parameter is not a reference to self, it is not a method, just an associated function
// Associated functions that are not methods could be things like constructors
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Constructs a Rectangle with equal width and height given only one parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 45,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 55,
    };

    let rect4 = Rectangle::square(15);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect4 is {:#?}", rect4);
}