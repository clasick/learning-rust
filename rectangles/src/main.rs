#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn fits(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let a = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area is {} for rectangle with values: {:?}",
        a.area(),
        a
    );

    let b = Rectangle {
        width: 5,
        height: 5,
    };

    match a.is_bigger(&b) {
        true => println!("Rectangle a is bigger than rectangle b."),
        false => println!("Rectangle a is NOT bigger than rectangle b."),
    }

    match b.fits(&a) {
        true => println!("Rectangle b fits rectangle a inside it."),
        false => println!("Rectangle b does NOT fit rectangle a inside it."),
    }

    let c = Rectangle {
        width: 10,
        height: 15
    };

    match c.fits(&b) {
        true => println!("Rectangle c fits rectangle b."),
        false => println!("Rectangle c does not fit rectangle b.")
    };
}
