enum Car {
    Wheels(u32),
    HorsePower
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
}
