fn largest_u32(array: &[u32]) -> u32 {
    let mut x = array[0];

    for &number in array.iter() {
        if number > x {
            x = number;
        }
    }

    x
}

fn largest_char(array: &[char]) -> char {
    let mut x = array[0];

    for &character in array.iter() {
        if character > x {
            x = character;
        }
    }

    x
}

// generic types

// fn largest<T>(array: &[T]) -> T {
//     let mut x = array[0];

//     for &item in array.iter() {
//         if item > x {
//             x = item;
//         }
//     }

//     x
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl <T,U> Point<T, U> {
    // using self here instead of &self moves the value so it can't be used again
    fn x(&self) -> &T {
        &self.x
    } 
}

impl <T,U> Point<T, U> {
    fn mix_up<V,W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y        
        }
    }
}

// generics in enum

enum Option<T, E> {
    Ok(T),
    Err(E)
}


fn main() {
    let x = vec![1, 5, 2, 3, 7];

    let x = largest_u32(&x);

    println!("{}", x);

    let x = vec!['a', 'z', 'c', 'd'];

    let x = largest_char(&x);

    println!("{}", x);

    let x = Point { x: 0, y: 1 };

    let x = Point { x: 0.0, y: 1.0};

    let x = Point { x: 0, y: 1.0};

    println!("{}", x.x());

    println!("{}", x.x());
    
    let c = x.mix_up(Point{x: 'a', y: 'b'});

    // this won't work because value was moved by the call above
    // println!("x: {}, y: {}", x.x, x.y);
    
    println!("x: {}, y: {}", c.x, c.y);

}
