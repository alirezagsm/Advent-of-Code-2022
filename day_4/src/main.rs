fn main() {
    let input = std::fs::read_to_string("src/input2.txt").expect("Error reading file");
    let bounds = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|rng| {
                    rng.split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_engulf = bounds
        .iter()
        .filter(|bnd| {
            (bnd[0][0] <= bnd[1][0] && bnd[0][1] >= bnd[1][1])
                || (bnd[1][0] <= bnd[0][0] && bnd[1][1] >= bnd[0][1])
        })
        .count();
    println!("Part 1: {num_engulf}");

    ///////////

    let num_no_engulf = bounds
        .iter()
        .filter(|bnd| (bnd[0][1] < bnd[1][0]) || (bnd[1][1] < bnd[0][0]))
        .count();
    let num_tot = bounds.len();
    let num_overlap = num_tot - num_no_engulf;
    println!("Part 2: {num_overlap}");
}
