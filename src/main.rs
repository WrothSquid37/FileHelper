use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_from_file(file_path: &str) -> Vec<String> {
    let file: Result<File, io::Error> = File::open(file_path);
    let reader: BufReader<Result<File, io::Error>> = BufReader::new(file);

    let str_vec: Vec<String>;
    for line in reader.lines() {

    }

    return str_vec;

}

// Test the acutally methods and functions used
fn main() {
    let c = read_from_file("input.txt");
    println!("{:?}", c);
}
