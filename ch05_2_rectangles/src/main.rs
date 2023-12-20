#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

// Naively you might do this:
fn area(rectangle: &Rectangle) -> f32 {
    rectangle.width * rectangle.height
}

// But implementing a method groups together useful functionality
impl Rectangle {
    fn area(&self) -> f32 {
        // very important to remember the '&' before self, we dont want ownership
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated functions don't take self as a parameter:
    // We use Self as an alias as one day the struct name might change
    // and this greatly reduces the associated hassle of such a change
    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }

    }
}



fn main() {
    // Note of course the naive approach does work:
    let rect1 = Rectangle {
        width: 12.,
        height: 594.,
    };
    let mut area1 = area(&rect1);
    println!("{area1}");

    // But much better is this:
    area1 = rect1.area();
    println!("{area1}");
    // Clearly will give the same result


    let rect2 = Rectangle {
        width: 11.,
        height: 10.,
    };

    let rect3 = Rectangle {
        width: 2.,
        height: 595.,
    };

    println!("Can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect 3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(12.);
    println!("square 1 is: {:?}", square1);
    println!("the area of square 1 is {}", square1.area());

}
