use std::collections::HashMap;

enum Car {
    Wheels(u32),
    HorsePower,
}

fn main() {
    let mut my_vector: Vec<i32> = vec![1, 2, 3];

    for x in &mut my_vector {
        *x *= 10;
    }

    my_vector.push(40);

    for x in &my_vector {
        println!("{}", x);
    }

    let mut my_vector: Vec<usize> = Vec::new();

    my_vector.push(5);
    my_vector.push(10);
    my_vector.push(15);
    my_vector.push(20);

    for x in &my_vector {
        println!("{}", x);
    }

    // The String Collection
    let x = String::new();
    let x = String::from("ABC");
    let mut x = "ABC".to_string();
    x.push_str("DEF");

    let x = String::from("Hello");
    let y = String::from("World");

    let z = x + &y;

    let z = format!("{} {}", z, y);

    let a = vec!['a', 'b', 'c'];
    let b = vec![1, 2, 3];

    let c: HashMap<_, _> = a.iter().zip(b.iter()).collect();

    let key = 'a';
    let a = match c.get(&key) {
        Some(x) => x,
        None => println!("Couldn't find the key"),
    };

    for (x, y) in c {
        println!("{} , {}", x, y);
    }

    c.insert(&'a', &2);

    c.entry(&'a').or_insert(&1);

    let x = "This is a string";

    let a = HashMap::new();

    for word in x.split_whitespace() {
        let count = a.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", a);
}
