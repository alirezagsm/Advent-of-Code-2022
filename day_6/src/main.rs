use std::collections::HashSet;

fn main() {
    let input = include_str!("input2.txt");
    let marker = get_marker(input, 4);
    println!("Part 1: {marker}");
    let marker = get_marker_with_iterators(input, 4);
    println!("Part 1: {marker}");

    let marker = get_marker(input, 14);
    println!("Part 2: {marker}");
    let marker = get_marker_with_iterators(input, 14);
    println!("Part 1: {marker}");
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
    0
}

fn get_marker_with_iterators(input: &str, length: usize) -> usize {
    let result = input.chars().collect::<Vec<_>>();
    let result = result
        .windows(length)
        .enumerate()
        .find(|(_i, buffer)| length == buffer.iter().collect::<HashSet<_>>().len());
    result.unwrap().0 + length
}
