use core::panic;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input2.txt");
    let mut T_position: Vec<(i32, i32)> = vec![];
    let mut Ti = 0;
    let mut Tj = 0;
    let mut Hi = 0;
    let mut Hj = 0;
    T_position.push((Ti, Tj));

    for line in input.lines() {
        let _line = line.split(" ").collect::<Vec<_>>();
        let dir = _line[0];
        let steps = _line[1].parse::<usize>().unwrap();

        // update T_position
        for _ in 0..steps {
            // dbg!(dir);
            match dir {
                "U" => Hj += 1,
                "D" => Hj -= 1,
                "L" => Hi -= 1,
                "R" => Hi += 1,
                _ => panic!("Invalid direction"),
            }
            get_Tij(&mut Ti, &mut Tj, Hi, Hj);
            T_position.push((Ti, Tj));
            // dbg!(Ti, Tj);
        }
    }
    // dbg!(&T_position);

    let T_position_unique = HashSet::<(i32, i32)>::from_iter(T_position);
    println!("Part1: {}", T_position_unique.len());

    // Part 2
    // initialize Knot positions at (0, 0)
    let num_knots = 10;
    let mut Hi = 0;
    let mut Hj = 0;
    let mut K_history = vec![];
    let mut K_positions = vec![(Hi, Hj); num_knots];
    K_history.push(K_positions.clone());
    let mut Leading_i = Hi;
    let mut Leading_j = Hj;
    let mut Ti = Hi;
    let mut Tj = Hj;

    for line in input.lines() {
        let _line = line.split(" ").collect::<Vec<_>>();
        let dir = _line[0];
        let steps = _line[1].parse::<usize>().unwrap();

        // update T_position

        for _ in 0..steps {
            dbg!(dir);
            match dir {
                "U" => Hj += 1,
                "D" => Hj -= 1,
                "L" => Hi -= 1,
                "R" => Hi += 1,
                _ => panic!("Invalid direction"),
            }
            K_positions[0] = (Hi, Hj);

            for k in 1..num_knots {
                Leading_i = K_positions[k - 1].0;
                Leading_j = K_positions[k - 1].1;
                Ti = K_positions[k].0;
                Tj = K_positions[k].1;
                get_Tij(&mut Ti, &mut Tj, Leading_i, Leading_j);
                K_positions[k] = (Ti, Tj);
            }
            K_history.push(K_positions.clone());
        }
        dbg!(&K_positions);

    }
    
    // dbg!(dbg2(&K_history));
    let tail_positions = K_history.iter().map(|x| x.last().unwrap().clone()).collect::<Vec<_>>();
    let tail_unique = HashSet::<(i32, i32)>::from_iter(tail_positions);
    println!("Part2: {}", tail_unique.len())
}

fn dbg(position: &Vec<(i32, i32)>) {
    for (i, j) in position {
        println!("({}, {})", i, j);
    }
}

fn dbg2(positions: &Vec<Vec<(i32, i32)>>) {
    for position in positions {
        println!("({position:?}");
    }
}

fn get_Tij(_Ti: &mut i32, _Tj: &mut i32, Hi: i32, Hj: i32) {
    // + mode
    let mut Ti = *_Ti;
    let mut Tj = *_Tj;
    let mut distance = 0;

    let quadrant;
    match Hi > Ti {
        true => match Hj > Tj {
            true => quadrant = 1,
            false => quadrant = 4,
        },
        false => match Hj > Tj {
            true => quadrant = 2,
            false => quadrant = 3,
        },
    }

    // perpendicular position
    if Ti == Hi || Tj == Hj {
        distance = (Hi - Ti) + (Hj - Tj);
        if Ti == Hi {
            Tj += distance - 1 * distance.signum();
        } else {
            Ti += distance - 1 * distance.signum();
        }
    } else {
        // diagonal position
        distance = (Hi - Ti).pow(2) + (Hj - Tj).pow(2);
        if distance > 2 {
            match quadrant {
                1 => {
                    Ti += 1;
                    Tj += 1;
                }
                2 => {
                    Ti -= 1;
                    Tj += 1;
                }
                3 => {
                    Ti -= 1;
                    Tj -= 1;
                }
                4 => {
                    Ti += 1;
                    Tj -= 1;
                }
                _ => panic!("Invalid quadrant"),
            }
        }
    }

    *_Ti = Ti;
    *_Tj = Tj;
}
