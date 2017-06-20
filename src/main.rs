extern crate titlecase;

use std::io::{self, BufRead, Write};
use titlecase::titlecase;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => println!("{}", titlecase(&line)),
            Err(error) => writeln!(io::stderr(), "{}\n", error)
                .expect("error writing error to stderr"),
        }
    }
}
