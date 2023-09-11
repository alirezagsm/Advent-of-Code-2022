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

    let mut nodes: Vec<(usize, usize)> = vec![];
    let mut nodes_candidates: Vec<Vec<(usize, usize)>> = vec![];
    let mut min_path = 999;
    let mut nclen = 0;
    let mut candidates;
    let mut back = false;

    loop {
        candidates = get_candidates(current, &map, &visited);
        if candidates.is_empty() {
            if nodes.is_empty() {
                break;
            }
            while visited.last().unwrap() != &nodes[nodes.len() - 1] {
                visited.pop();
            }
            nclen = nodes_candidates.len();
            if nodes_candidates[nclen - 1].len() == 0 {
                nodes.pop();
                nodes_candidates.pop();
            } else if nodes_candidates[nclen - 1].len() > 0 {
                current = nodes_candidates[nclen - 1].pop().unwrap();
            }
        } else {
            current = candidates.pop().unwrap();
        }

        if candidates.len() > 0 {
            nodes.push(current);
            nodes_candidates.push(candidates);
        }
        visited.push(current);
        if current == end {
            dbg!(&visited.len());
            if visited.len() < min_path {
                min_path = visited.len();
                show_map(&map, &visited);
                dbg!(&min_path);
            }
        }
    }

    println!("Part 1: {}", min_path)
}

fn show_map(map: &Vec<Vec<u32>>, visited: &Vec<(usize, usize)>) {
    // parse map into string
    let mut map: Vec<Vec<String>> = map
        .iter()
        .map(|row| row.iter().map(|&c| c.to_string()).collect())
        .collect();
    let visited_len = visited.len();
    for n in 0..visited_len - 2 {
        if visited[n].0 as i32 == visited[n + 1].0 as i32 - 1 {
            map[visited[n].0][visited[n].1] = "v".to_string();
        } else if visited[n].0 as i32 == visited[n + 1].0 as i32 + 1 {
            map[visited[n].0][visited[n].1] = "^".to_string();
        } else if visited[n].1 as i32 == visited[n + 1].1 as i32 - 1 {
            map[visited[n].0][visited[n].1] = ">".to_string();
        } else if visited[n].1 as i32 == visited[n + 1].1 as i32 + 1 {
            map[visited[n].0][visited[n].1] = "<".to_string();
        }
    }
    for row in map {
        for col in row {
            print!("{:4}", col);
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
