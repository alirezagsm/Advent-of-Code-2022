use std::sync::Arc;

fn main() {
    let input_raw = include_str!("input1.txt");
    let input = input_raw.split("\n\n").collect::<Vec<_>>();
    let input_stack = input[0];
    let input_moves = input[1];
    let n_stacks = input_stack.lines().collect::<Vec<_>>()[0].chars().count() / 4 + 1;
    println!("n_stacks = {n_stacks}");

    let mut stacks = vec![vec![]; n_stacks + 1];

    input_stack.lines().map(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .as_slice()
            .enumerate()
            .map(|(i, item)| {
                stacks[i].push(item);
            })
    });

    // let stacks = vec![vec![]; n_stacks + 1];

    println!("{stacks:?}");

    //// better way to do this would be:
    // let result = input_stack
    //     .lines()
    //     .map(|line| {
    //         line.chars()
    //             .enumerate()
    //             .flat_map(|(i, c)| {
    //                 if i != 0 && i % 4 == 0 {
    //                     Some(' ')
    //                 } else {
    //                     None
    //                 }
    //                 .into_iter()
    //                 .chain(std::iter::once(c))
    //             })
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<_>>();

    // println!("{result:?}");
}
