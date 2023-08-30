use std::path::Path;

fn main() {
    //read input.txt line by line
    let filepath = Path::new("src/input.txt");
    let input = std::fs::read_to_string(filepath).expect("Error reading file");
    let collection: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let _vec: Vec<&str> = Vec::new();
        if line == "" {
            break;
        }
        _vec.push(line);
    }
}
