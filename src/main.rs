extern crate tico;

use std::io::Write;

fn main() {
    let path = std::env::args().nth(1).unwrap_or(String::from(""));

    std::io::stdout()
        .write(tico::tico(&path).as_bytes())
        .unwrap();
}
