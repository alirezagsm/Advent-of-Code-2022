fn get_value(c: char) -> i32 {
    let mut num = 0;
    if c.is_ascii_lowercase() {
        num = c as i32;
        num -= 96;
    } else if c.is_ascii_uppercase() {
        num = c as i32;
        num -= 64 - 26;
    }
    num
}

fn main() {
    let input = std::fs::read_to_string("src/input2.txt").unwrap();
    let mut dup: Vec<char> = Vec::new();
    for line in input.lines() {
        // let numbers: Vec<i32> = line.chars().map(|c| get_value(c)).collect();

        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..line.len()];

        for c in first.chars() {
            if second.contains(c) {
                dup.push(c);
                break;
            }
        }
    }
    let value = dup
        .iter()
        .map(|c| get_value(*c))
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
    println!("Part 1 result: {value:?}");

    //////////////////////////////////////////////////

    let mut dup: Vec<char> = Vec::new();
    for group in input.lines().collect::<Vec<_>>().chunks(3) {
        'brk: for c0 in group[0].chars() {
            for c1 in group[1].chars() {
                for c2 in group[2].chars() {
                    if c0 == c1 && c1 == c2 {
                        dup.push(c0);
                        break 'brk;
                    }
                }
            }
        }
    }
    let value = dup
        .iter()
        .map(|c| get_value(*c))
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
    println!("Part 2 result: {value:?}");
}
