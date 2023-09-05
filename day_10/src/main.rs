fn main() {
    let input = include_str!("input1.txt");
    let mut x = 1;
    let mut x_new = x;
    let mut cycle = 1;
    let mut signal_strength = 0;
    let mut midcycle = false;
    let mut boolprint = true;
    let read_cycles = vec![20, 60, 100, 140, 180, 220];
    for line in input.lines() {
        if line.contains("noop") {
            cycle += 1;
        } else if line.contains("addx") {
            x_new += line.split(" ").last().unwrap().parse::<i32>().unwrap();
            cycle += 1;
            midcycle = true;
        }
        if read_cycles.contains(&cycle) && boolprint {
            signal_strength += x * cycle;
            dbg!(cycle, line, x, x * cycle, signal_strength);
            boolprint = false;
        }
        if midcycle {
            x = x_new;
            cycle += 1;
            midcycle = false;
        }
        if read_cycles.contains(&cycle) && boolprint {
            signal_strength += x * cycle;
            dbg!(cycle, line, x, x * cycle, signal_strength);
        }
        boolprint = true;
    }
    dbg!(signal_strength);

    // Part 2

    let mut pixels_flat = vec![0; 240];
    pixels_flat[0] = 1;

    let mut x = 1;
    let mut x_new = x;
    let mut cycle = 1;
    let mut midcycle = false;
    for line in input.lines() {
        if line.contains("noop") {
            cycle += 1;
        } else if line.contains("addx") {
            x_new += line.split(" ").last().unwrap().parse::<i32>().unwrap();
            cycle += 1;
            switch(&mut pixels_flat, x, cycle);
            midcycle = true;
        }
        if midcycle {
            x = x_new;
            cycle += 1;
            switch(&mut pixels_flat, x, cycle);
            midcycle = false;
        }
        switch(&mut pixels_flat, x, cycle);
    }

    let pixels = pixels_flat
        .chunks(40)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<i32>>>();
    display(pixels);
}

fn switch(pixels: &mut Vec<i32>, x: i32, cycle: i32) {
    // dbg!(x, cycle);
    if (((cycle) % 40 - 1) - x ).abs() < 2 {
        pixels[cycle as usize -1] = 1;
        // dbg!("draw", cycle as usize -1);
    }
}

fn display(pixels: Vec<Vec<i32>>) {
    for row in pixels {
        for pixel in row {
            if pixel == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
