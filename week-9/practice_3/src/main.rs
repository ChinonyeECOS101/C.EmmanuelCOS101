use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could delete file");
    println!("File sucessfully removed");
}
