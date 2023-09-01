#[cfg(windows)]
const EMPTYLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTYLINE: &str = "\n\n";

fn main() {
    let input_raw = include_str!("input2.txt");
    let input = input_raw.split(EMPTYLINE).collect::<Vec<_>>();
    let input_stack = input[0];
    let input_moves = input[1];
    let n_stacks = input_stack.lines().collect::<Vec<_>>()[0].chars().count() / 4 + 1;
    println!("n_stacks = {n_stacks}");

    let mut stacks: Vec<Vec<String>> = vec![vec![]; n_stacks];

    input_stack
        .lines()
        .rev()
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|c| c.iter().filter(|&c| c.is_alphabetic()).collect::<String>())
                .enumerate()
                .for_each(|(i, item)| stacks[i].push(item.clone()));
        })
        .for_each(drop);

    let stacks_base = stacks
        .iter()
        .map(|stack| stack.join(""))
        .filter(|stack| !stack.is_empty())
        .collect::<Vec<_>>();

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

    let mut moves_base: Vec<Vec<u32>> = vec![];

    input_moves
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|w| w.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .for_each(|word| moves_base.push(word));

    // println!("{moves:?}");

    // println!("{stacks_base:?}");
    let mut stacks = stacks_base.clone();
    let moves = moves_base.clone();
    for action in moves {
        let num = action[0];
        let from = (action[1] - 1) as usize;
        let to = (action[2] - 1) as usize;

        let mut item = ' ';
        for _ in 0..num {
            item = stacks[from].pop().unwrap();
            stacks[to].push(item);
            // println!("{stacks:?}");
        }
    }

    let result = stacks
        .iter()
        .map(|s| s.chars().rev().collect::<Vec<_>>()[0])
        .collect::<String>();
    println!("result: {result:?}");

    // Part 2

    let mut stacks = stacks_base.clone();
    let moves = moves_base.clone();

    for action in moves {
        let num = action[0] as usize;
        let from = (action[1] - 1) as usize;
        let to = (action[2] - 1) as usize;

        let fromlen = stacks[from].len();
        let items = stacks[from]
            .drain(fromlen - num..fromlen)
            .collect::<Vec<_>>();
        stacks[to].extend(items.clone());
    }

    let result = stacks
        .iter()
        .map(|s| s.chars().rev().collect::<Vec<_>>()[0])
        .collect::<String>();
    println!("result: {result:?}");
}
