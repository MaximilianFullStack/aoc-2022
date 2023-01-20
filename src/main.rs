use std::fs;

fn visi(grid: String) -> usize {
   
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", visi(contents));
}