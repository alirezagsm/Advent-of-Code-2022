// use std::cell::{Cell, RefCell};
// use std::sync::Mutex;

// fn main() {
//     let mut myvec = Mutex::new(vec![1, 2, 3, 4, 5]);

//     for v in myvec.get_mut() {
//         dbg!(v);
//         // myvec.get_mut()[0]= 10;
//     }
// }

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    item_operations: String,
    item_value: i32,
    test_value: i32,
    true_destination: usize,
    false_destination: usize,
    count: u32,
}

// implement print for Monkey
impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monkey {{ items: {:?}}}", self.items)
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let mut monkeys = Vec::<Monkey>::new();
    // initialize monkey vector
    let mut _items: Vec<i32> = Vec::new();
    let mut _operation: &str;
    let mut _item_operation: &str = "";
    let mut _item_value: i32 = 0;
    let mut _test_value: i32 = 0;
    let mut _true_destination: usize = 0;
    let mut _false_destination: usize = 0;
    for line in input.lines() {
        if line.contains("Starting") {
            _operation = line.split("items: ").last().unwrap();
            _items = _operation
                .split(", ")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
        } else if line.contains("Operation") {
            _operation = line.split("new = ").last().unwrap();
            for word in _operation.split_whitespace() {
                if word.contains("+") {
                    _item_operation = "+";
                } else if word.contains("*") {
                    _item_operation = "*";
                } else if word.chars().any(|c| c.is_ascii_digit()) {
                    _item_value = word.parse::<i32>().unwrap();
                } else {
                    _item_value = 0;
                }
            }
        } else if line.contains("Test") {
            _operation = line.split("divisible by ").last().unwrap();
            _test_value = _operation.parse::<i32>().unwrap();
        } else if line.contains("true") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _true_destination = _operation.parse::<usize>().unwrap();
        } else if line.contains("false") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _false_destination = _operation.parse::<usize>().unwrap();
        }
        if line.is_empty() {
            monkeys.push(Monkey {
                items: _items.clone(),
                item_operations: _item_operation.to_string(),
                item_value: _item_value,
                test_value: _test_value,
                true_destination: _true_destination,
                false_destination: _false_destination,
                count: 0,
            });
        }
    }

    let mut round_count = 0;
    while round_count < 20 {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                monkeys[m].count += 1;
                let destination;
                let item: &mut i32 = &mut 0;
                {
                    let monkey = monkeys.get_mut(m).unwrap();
                    let item = monkey.items.get_mut(i).unwrap();
                    if monkey.item_operations == "+" {
                        *item += monkey.item_value;
                    } else if monkey.item_operations == "*" {
                        *item *= monkey.item_value;
                    }
                    *item /= 3;

                    if *item % monkey.test_value == 0 {
                        destination = monkey.true_destination;
                    } else {
                        destination = monkey.false_destination;
                    }
                    monkey.items.remove(i);
                }
                monkeys.get_mut(destination).unwrap().items.push(*item);
            }
        }
        round_count += 1;
    }

    dbg!(monkeys);
}
