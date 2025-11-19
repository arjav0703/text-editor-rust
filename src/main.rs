use std::io::{self, Read, Write};

fn main() {
    for c in io::stdin().bytes() {
        let c = c.unwrap() as char;
        println!("{}", c);
    }
}
