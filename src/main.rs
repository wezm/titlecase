extern crate titlecase;

use std::io::{self, BufRead};
use titlecase::titlecase;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => println!("{}", titlecase(&line)),
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
