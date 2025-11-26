use std::fs;
use std::io::ErrorKind;

fn main() {
    if let Err(e) = fs::remove_file("data.txt") {
        if e.kind() != ErrorKind::NotFound {
            panic!("could not remove file: {}", e);
        }
    }
    println!("file removed or did not exist");
}