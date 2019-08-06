extern crate tico;

use std::io::Write;

fn main() {
    let path = std::env::args().nth(1).unwrap_or(String::from(""));
    let home_dir = dirs::home_dir().unwrap();
    let home_dir_str = home_dir.to_str();

    std::io::stdout()
        .write(tico::tico(&path, home_dir_str).as_bytes())
        .unwrap();
}
