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
}
