#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, other : &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let a = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The area is {} for rectangle with values: {:?}", a.area(), a);

    let b = Rectangle {
        width: 5,
        height: 5
    };

    match a.is_bigger(&b) {
        true => {
            println!("Rectangle a is bigger than rectangle b.")
        },
        false => {
            println!("Rectangle a is NOT bigger than rectangle b.")
        }
    }
}