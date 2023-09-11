fn main() {
    let input = include_str!("input1.txt");

    let mut map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u32).collect())
        .collect();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 83).map(|j| (i, j)))
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 69).map(|j| (i, j)))
        .unwrap();

    map[start.0][start.1] = 96;
    map[end.0][end.1] = 123;
    let mut current = start;
    let mut visited = vec![current];

    let mut candidates = get_candidates(current, &map, &visited);
    let mut nodes: Vec<(usize, usize)>;
    let mut nodes_explored: Vec<Vec<(usize, usize)>>;
    let mut min_path = 999;

    loop {
        candidates = get_candidates(current, &map, &visited);
        if candidates.is_empty() {
            match nodes.pop() {
                Some(node) => {
                    current = node;
                    let mut history = current;
                    while visited.last().unwrap() != &current {
                        history = visited.pop().unwrap();
                    }
                    visited.pop();
                    visited.push(history);
                }
                None => break,
            }
        } else {
            // go back
            if candidates.len() > 1 {
                nodes.push(current);
            }
            current = candidates.pop().unwrap();
        };

        visited.push(current);
        show_map(&map, &visited);
        if current == end && visited.len() < min_path {
            min_path = visited.len();
            dbg!(&visited.len());
        }
    }

    println!("Part 1: {}", min_path - extras)
}

fn show_map(map: &Vec<Vec<u32>>, visited: &Vec<(usize, usize)>) {
    // replace map element with 0 for visited
    let mut map = map.clone();
    for (i, j) in visited {
        map[*i][*j] = 0;
    }
    for row in map {
        for col in row {
            print!("{:5}", col);
        }
        println!("");
    }
    println!("");
}

fn get_candidates(
    current: (usize, usize),
    map: &Vec<Vec<u32>>,
    visited: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let (i, j) = current;
    let mut candidates = vec![];

    for (ii, jj) in [
        (i as i32 - 1, j as i32),
        (i as i32 + 1, j as i32),
        (i as i32, j as i32 - 1),
        (i as i32, j as i32 + 1),
    ] {
        if ii >= 0
            && jj >= 0
            && ii < map.len() as i32
            && jj < map[0].len() as i32
            && !visited.contains(&(ii as usize, jj as usize))
            && map[ii as usize][jj as usize] <= map[i][j] + 1
            && map[ii as usize][jj as usize] >= map[i][j] - 1
        {
            candidates.push((ii as usize, jj as usize));
        }
    }
    candidates
}

fn get_next(current: (usize, usize), input: &Vec<Vec<char>>) {
    todo!();
    // let candidates = get_candidates(current, input);
    // let filtered_candidates = candidates
    //     .iter()
    //     .filter(|(i, j)| input[*i][*j] != '#')
    //     .collect();
    // input[filtered_candidates[0].0][filtered_candidates[0].1] = '#';
    // let mut next = get_next(filtered_candidates[0], input);
}
