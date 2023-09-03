use ndarray::{s, Array};

fn main() {
    let input = include_str!("input2.txt");
    let heigh_map = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let nrows = heigh_map.len();
    let ncols = heigh_map[0].len();
    let edge_len = nrows * ncols - 4;

    let height_array = Array::from_shape_vec(
        (nrows, ncols),
        heigh_map.iter().flatten().collect::<Vec<_>>().clone(),
    )
    .unwrap();

    let mut visible_array = Array::<usize, _>::zeros(height_array.dim()); //raw_dim

    // look at height_map from 4 directions

    for r in 0..nrows {
        //ndarray sucks and can't do window bc of trait bounds
        let height_vec = height_array.slice(s![r, ..]).to_vec();
        let mut max_height = &-1;
        for c in 0..height_vec.len() {
            let height = height_vec[c];
            // println!("{} {}", height, max_height);
            if height > max_height {
                visible_array[[r, c]] += 1;
                max_height = height;
            }
            // dbg!(&height_vec);
            // dbg!(&visible_array);
        }
    }

    for c in 0..ncols {
        //ndarray sucks and can't do window bc of trait bounds
        let height_vec = height_array.slice(s![.., c]).to_vec();
        let mut max_height = &-1;
        for r in 0..height_vec.len() {
            let height = height_vec[r];
            // println!("{} {}", height, max_height);
            if height > max_height {
                visible_array[[r, c]] += 1;
                max_height = height;
            }
            // dbg!(&height_vec);
            // dbg!(&visible_array);
        }
    }

    for r in 0..nrows {
        //ndarray sucks and can't do window bc of trait bounds
        let height_vec0 = height_array.slice(s![r, ..]).to_vec();
        let height_vec = height_vec0.iter().rev().collect::<Vec<_>>();
        let veclen = height_vec.len();
        let mut max_height = &&-1;
        for c in 0..height_vec.len() {
            let height = height_vec[c];
            // println!("{} {}", height, max_height);
            if height > max_height {
                visible_array[[r, veclen - 1 - c]] += 1;
                max_height = height;
            }
            // dbg!(&height_vec);
            // dbg!(&visible_array);
        }
    }

    for c in 0..ncols {
        //ndarray sucks and can't do window bc of trait bounds
        let height_vec0 = height_array.slice(s![.., c]).to_vec();
        let height_vec = height_vec0.iter().rev().collect::<Vec<_>>();
        let veclen = height_vec.len();
        let mut max_height = &&-1;
        for r in 0..height_vec.len() {
            let height = height_vec[r];
            // println!("{} {}", height, max_height);
            if height > max_height {
                visible_array[[veclen - 1 - r, c]] += 1;
                max_height = height;
            }
            // dbg!(&height_vec);
            // dbg!(&visible_array);
        }
    }

    let result = visible_array
        .iter()
        .filter(|&&x| x > 0)
        .collect::<Vec<_>>()
        .len();
    println!("Part1: {result}");

    // Part 2

    let mut scenic_score = Array::<usize, _>::ones(height_array.dim()); //raw_dim

    // look in 4 directions from each tree

    let mut distance;
    let mut max_height;
    let mut threshold;
    for r0 in 0..nrows {
        for c0 in 0..ncols {
            // dbg!(r0, c0);
            let height_vec0 = height_array.slice(s![r0, ..]).to_vec();
            let base_height = height_array[[r0, c0]];

            // dbg!("look right");
            distance = 0;
            max_height = *base_height;
            threshold = 0;
            for c in (c0 + 1..ncols) {
                fun1(
                    base_height,
                    height_vec0[c],
                    &mut threshold,
                    &mut max_height,
                    &mut distance,
                    c,
                    c0,
                );
            }
            // dbg!(distance);
            scenic_score[[r0, c0]] *= distance;

            // dbg!("look left");
            distance = 0;
            max_height = *base_height;
            threshold = 0;
            for c in (0..c0).rev() {
                fun1(
                    base_height,
                    height_vec0[c],
                    &mut threshold,
                    &mut max_height,
                    &mut distance,
                    c,
                    c0,
                );
            }
            // dbg!(distance);
            scenic_score[[r0, c0]] *= distance;

            let height_vec0 = height_array.slice(s![.., c0]).to_vec();

            // dbg!("look down");
            distance = 0;
            max_height = *base_height;
            threshold = 0;
            for r in (r0 + 1..nrows) {
                fun1(
                    base_height,
                    height_vec0[r],
                    &mut threshold,
                    &mut max_height,
                    &mut distance,
                    r,
                    r0,
                );
            }
            // dbg!(distance);
            scenic_score[[r0, c0]] *= distance;

            // dbg!("look up");
            distance = 0;
            max_height = *base_height;
            threshold = 0;
            for r in (0..r0).rev() {
                fun1(
                    base_height,
                    height_vec0[r],
                    &mut threshold,
                    &mut max_height,
                    &mut distance,
                    r,
                    r0,
                );
            }
            // dbg!(distance);
            scenic_score[[r0, c0]] *= distance;
        }
    }
    // dbg!(&scenic_score);

    let result = scenic_score.iter().max().unwrap();
    println!("Part2: {result}");
}

fn fun1(
    base_height: &i32,
    height_vec0: &i32,
    threshold: &mut i32,
    max_height: &mut i32,
    distance: &mut usize,
    c: usize,
    c0: usize,
) {
    if height_vec0 < base_height && *threshold == 0 {
        // dbg!("+short");
        *distance += 1;
        *max_height = **[max_height, base_height, height_vec0].iter().max().unwrap();
        return;
    }
    *threshold += 1;
    if height_vec0 == max_height && *threshold == 1 {
        // dbg!("+same");
        *distance += 1;
        *threshold += 1;
    }
    if height_vec0 > &max_height && *threshold >= 1 {
        // dbg!("+max");
        *distance += 1;
        *threshold += 1;
        *max_height = **[max_height, base_height, height_vec0].iter().max().unwrap();
    }
    if c == c0 + 1 && *distance == 0 {
        // dbg!("+1");
        *distance += 1;
        *max_height = **[max_height, base_height, height_vec0].iter().max().unwrap();
    }
}

// dbg!(&base_height);
// dbg!(height_vec0[c]);
// if height_vec0[c] < &base_height && threshold == 0 {
//     dbg!("+short");
//     distance += 1;
//     max_height = *[max_height, *base_height, *height_vec0[c]]
//         .iter()
//         .max()
//         .unwrap();
//     continue;
// }
// threshold += 1;
// dbg!("threshold");
// if height_vec0[c] == &max_height && threshold == 1 {
//     dbg!("+same");
//     distance += 1;
//     threshold += 1;
// }
// if height_vec0[c] > &max_height && threshold >= 1 {
//     dbg!("+max");
//     distance += 1;
//     threshold += 1;
//     max_height = *[max_height, *base_height, *height_vec0[c]]
//         .iter()
//         .max()
//         .unwrap();
// }
// if c == c0 + 1 && distance == 0 {
//     distance += 1;
//     max_height = *[max_height, *base_height, *height_vec0[c]]
//         .iter()
//         .max()
//         .unwrap();
//     dbg!("+1");
// }
