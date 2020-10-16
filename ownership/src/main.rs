fn main() {
    let str_literal = "blinding lights";

    let string_type = String::from("in your eyes");

    let _x = &string_type;

    println!("{}", nth_word(str_literal, 1));

    println!("{}", nth_word(&string_type, 2));

    println!("{}", nth_word(&string_type, 3)); // doesn't work?
}

fn nth_word(x: &str, n: u32) -> &str {
    let mut n = n;
    let mut prev_blank = 0;

    for (i, c) in x.bytes().enumerate() {
        if b' ' == c {
            n = n - 1;

            if n == 0 {
                return &x[prev_blank..i];
            }
            prev_blank = i+1;
        }
    }
    &x[..]
}