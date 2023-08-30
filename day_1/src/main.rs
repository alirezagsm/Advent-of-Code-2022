use std::path::Path;

fn main() {
    let filepath = Path::new("src/input.txt");
    let input = std::fs::read_to_string(filepath).expect("Error reading file");
    let mut collection: Vec<Vec<i32>> = Vec::new();
    let mut _vec: Vec<i32> = Vec::new();
    for line in input.lines() {
        let line_int = line.parse::<i32>();
        match line_int {
            Ok(line_int) => {
                _vec.push(line_int);
            }
            Err(_) => {
                collection.push(_vec.clone());
                _vec.clear();
            }
        }
    }
    let mut max_carbs = 0;
    for item in collection {
        let carbs = item.iter().sum();
        if carbs > max_carbs {
            max_carbs = carbs;
        }
    }
    println!("Max carbs: {}", max_carbs)
}
