extern crate rand;

use rand::Rng;

fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1, 101);
    
    println!("Guess a number!");

    loop {

        let mut guessed_number = String::new();
    
        std::io::stdin()
            .read_line(&mut guessed_number)
            .expect("Unable to read the number!");
    
        let guessed_number: u32 = match guessed_number
            .trim()
            .parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Skipping invalid input... Try again!");
                    continue;
                }
            };
    
        match guessed_number.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("{} is lesser", guessed_number),
            std::cmp::Ordering::Equal => {
                println!("{} is equal", guessed_number);
                break();
            },
            std::cmp::Ordering::Greater => println!("{} is greater", guessed_number),
        }
    }
}
