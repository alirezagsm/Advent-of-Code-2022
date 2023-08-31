
fn main() {
    let input = std::fs::read_to_string("src/input2.txt").unwrap();
    let mut score = 0;
    for line in input.lines() {
        let _line = line.split(" ").collect::<Vec<&str>>();
        let p1 = _line[0];
        let p2 = _line[1];
        if p1 == "A" {
            if p2 == "X" {
                score += 1;
                score += 3;
            } else if p2 == "Y"{
                score += 2;
                score += 6;
            } else if p2 == "Z" {
                score += 3;
                score += 0;
            }
        }
        else if p1 == "B" {
            if p2 == "X" {
                score += 1;
                score += 0;
            } else if p2 == "Y"{
                score += 2;
                score += 3;
            } else if p2 == "Z" {
                score += 3;
                score += 6;
            }
        }
        else if p1 == "C" {
            if p2 == "X" {
                score += 1;
                score += 6;
            } else if p2 == "Y"{
                score += 2;
                score += 0;
            } else if p2 == "Z" {
                score += 3;
                score += 3;
            }
        }
    }
    println!("Part 1: {}", score);
    // Part Two
    let mut score = 0;
    for line in input.lines() {
        let _line = line.split(" ").collect::<Vec<&str>>();
        let p1 = _line[0];
        let p2 = _line[1];
        if p1 == "A" {
            if p2 == "X" {
                score += 3;
                score += 0;
            } else if p2 == "Y"{
                score += 1;
                score += 3;
            } else if p2 == "Z" {
                score += 2;
                score += 6;
            }
        }
        else if p1 == "B" {
            if p2 == "X" {
                score += 1;
                score += 0;
            } else if p2 == "Y"{
                score += 2;
                score += 3;
            } else if p2 == "Z" {
                score += 3;
                score += 6;
            }
        }
        else if p1 == "C" {
            if p2 == "X" {
                score += 2;
                score += 0;
            } else if p2 == "Y"{
                score += 3;
                score += 3;
            } else if p2 == "Z" {
                score += 1;
                score += 6;
            }
        }
    }
    println!("Part 2: {}", score);
}
