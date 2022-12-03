// Implementing a rectangle struct with width and height fields
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Takes a reference to a Rectangle struct and returns the area of the rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
