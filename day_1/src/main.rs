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
    collection.push(_vec.clone());

    let mut carb_vec = collection
        .iter()
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    carb_vec.sort();
    println!("{carb_vec:?}");
    println!("Max carb: {}", carb_vec.last().unwrap());
    print!(
        "The sum of the three largest numbers is: {}\n",
        carb_vec.iter().rev().take(3).sum::<i32>()
    );
}
