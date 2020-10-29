use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("Hello, world!");

    // let x = vec![1, 2, 3];

    // println!("{}", x[3]);

    // let x = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => panic!("Exiting with error: {}", error)
    // };

    let x = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(ref error) => panic!("Couldn't create file!: {:?}", error),
        },
        Err(error) => panic!("Couldn't open file! {:?}", error),
    };
}
