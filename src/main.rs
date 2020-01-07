use std::env;

fn print_bottles() {
    for x in (1..99).rev() {
        println!("{} bottles on the wall", x);
    }
}

fn main() {
    let source = match env::args().skip(1).take(1).next() {
        Some(val) => val.to_uppercase(),
        None => String::new(),
    };

    for arg in source.chars() {
        let mut acc: u8 = 0;
        match arg {
            'H' => println!("{}", "Hello, world!"),
            'Q' => println!("{:?}", source),
            '9' => print_bottles(),
            '+' => acc += 1,
            _ => println!("{} {}", "Illegal character", arg),
        };
    }
}
