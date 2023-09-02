use std::collections::HashSet;

fn main() {
    let input = include_str!("input2.txt");
    let marker = get_marker(input, 4);
    println!("Part 1: {marker}");

    let marker = get_marker(input, 14);
    println!("Part 2: {marker}");
}

fn get_marker(input: &str, length: usize) -> usize {
    let mut buffer = input[0..length].chars().collect::<Vec<_>>();
    for (i, c) in input[length..].chars().enumerate() {
        if buffer.len() == length {
            let unique = buffer.iter().collect::<HashSet<_>>();
            if unique.len() == length {
                return i + length;
            }
        }
        buffer.push(c);
        buffer.remove(0);
    }
    1
}

