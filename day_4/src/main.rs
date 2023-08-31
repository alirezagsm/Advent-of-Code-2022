fn main() {
    let input = std::fs::read_to_string("src/input2.txt").expect("Error reading file");
    let bounds = input
        .lines()
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<_>>();
            let range1 = ranges[0].split('-').collect::<Vec<_>>();
            let range2 = ranges[1].split('-').collect::<Vec<_>>();
            (range1, range2)
        })
        .collect::<Vec<_>>();

    let num_engulf = bounds
        .iter()
        .filter(|(bnd1, bnd2)| {
            let b10 = bnd1[0].parse::<u32>().unwrap();
            let b11 = bnd1[1].parse::<u32>().unwrap();
            let b20 = bnd2[0].parse::<u32>().unwrap();
            let b21 = bnd2[1].parse::<u32>().unwrap();

            (b10 <= b20 && b11 >= b21) || (b20 <= b10 && b21 >= b11)
        })
        .count();
    println!("Part 1: {num_engulf}");

    ///////////

    let num_no_engulf = bounds
        .iter()
        .filter(|(bnd1, bnd2)| {
            let b10 = bnd1[0].parse::<u32>().unwrap();
            let b11 = bnd1[1].parse::<u32>().unwrap();
            let b20 = bnd2[0].parse::<u32>().unwrap();
            let b21 = bnd2[1].parse::<u32>().unwrap();

            (b11 < b20) || (b21 < b10)
        })
        .count();
    let num_tot = bounds.len();
    let num_overlap = num_tot - num_no_engulf;
    println!("Part 2: {num_overlap}");
}
