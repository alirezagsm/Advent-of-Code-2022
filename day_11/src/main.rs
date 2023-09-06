// use std::cell::{Cell, RefCell};
// use std::sync::Mutex;

// fn main() {
//     let mut myvec = Mutex::new(vec![1, 2, 3, 4, 5]);

use core::panic;
use std::collections::{BTreeMap, BTreeSet};

//     for v in myvec.get_mut() {
//         dbg!(v);
//         // myvec.get_mut()[0]= 10;
//     }
// }
use num::bigint::BigInt;

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
        write!(f, "Monkey {{ items: {:?}}}", self.items.len())
    }
}

fn main() {
    let input = include_str!("input2.txt");
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
                if monkeys[m].items[i] == 0 {
                    continue;
                }
                monkeys[m].count += 1;
                let destination;
                let mut item: i32;

                let monkey = monkeys.get_mut(m).unwrap();
                item = monkey.items[i];
                if monkey.item_operations == "+" {
                    if monkey.item_value == 0 {
                        item += item;
                    } else {
                        item += monkey.item_value;
                    }
                } else if monkey.item_operations == "*" {
                    if monkey.item_value == 0 {
                        item *= item;
                    } else {
                        item *= monkey.item_value;
                    }
                }
                item /= 3;
                if item % monkey.test_value == 0 {
                    destination = monkey.true_destination;
                } else {
                    destination = monkey.false_destination;
                }
                monkey.items[i] = 0;
                monkeys.get_mut(destination).unwrap().items.push(item);
            }
        }
        round_count += 1;
        dbg!(round_count);
    }

    for monkey in monkeys.iter() {
        println!("{}", monkey.count)
    }

    let mut active_monkeys = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    active_monkeys.sort();
    dbg!(&active_monkeys);
    let result = active_monkeys.iter().rev().take(2).product::<u32>();
    println!("Part1: {result}");

    //Part 2
    struct Monkey_Big {
        items: BTreeSet<String>,
        item_operations: String,
        item_value: u64,
        test_value: u64,
        true_destination: usize,
        false_destination: usize,
        count: u64,
    }
    let mut monkeys = Vec::<Monkey_Big>::new();
    // initialize monkey vector
    let mut _items: BTreeSet<String> = BTreeSet::new();
    let mut _operation: &str;
    let mut _item_operation: &str = "";
    let mut _item_value: u64 = 0;
    let mut _test_value: u64 = 0;
    let mut _true_destination: usize = 0;
    let mut _false_destination: usize = 0;
    for line in input.lines() {
        if line.contains("Starting") {
            _operation = line.split("items: ").last().unwrap();
            _items = _operation
                .split(", ")
                .map(|s| s.parse::<String>().unwrap())
                .collect();
        } else if line.contains("Operation") {
            _operation = line.split("new = ").last().unwrap();
            for word in _operation.split_whitespace() {
                if word.contains("+") {
                    _item_operation = "+";
                } else if word.contains("*") {
                    _item_operation = "*";
                } else if word.chars().any(|c| c.is_ascii_digit()) {
                    _item_value = word.parse::<u64>().unwrap();
                } else {
                    _item_value = 0;
                }
            }
        } else if line.contains("Test") {
            _operation = line.split("divisible by ").last().unwrap();
            _test_value = _operation.parse::<u64>().unwrap();
        } else if line.contains("true") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _true_destination = _operation.parse::<usize>().unwrap();
        } else if line.contains("false") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _false_destination = _operation.parse::<usize>().unwrap();
        }
        if line.is_empty() {
            monkeys.push(Monkey_Big {
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

    // add 0 to items of all monkeys
    for monkey in monkeys.iter_mut() {
        monkey.items.insert("0".to_string());
    }

    let mut round_count = 0;
    let crt = monkeys.iter().map(|m| m.test_value).product::<u64>();
    dbg!(crt);
    while round_count < 10_000 {
        for m in 0..monkeys.len() {
            let tree_len = monkeys[m].items.len();
            let item_string_vec = monkeys[m]
                .items
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for i in 0..item_string_vec.len() {
                let item = item_string_vec[i].clone();
                let item_monkey = monkeys[m].items.get(&item).unwrap().clone();
                if item_monkey == "0" {
                    continue;
                }
                monkeys[m].count += 1;
                let destination;
                let monkey = monkeys.get_mut(m).unwrap();
                let mut item_bigint = item_monkey.parse::<BigInt>().unwrap();
                if monkey.item_operations == "+" {
                    if monkey.item_value == 0 {
                        item_bigint += item_bigint.clone();
                    } else {
                        item_bigint += monkey.item_value;
                    }
                } else if monkey.item_operations == "*" {
                    if monkey.item_value == 0 {
                        item_bigint *= item_bigint.clone();
                    } else {
                        item_bigint *= monkey.item_value;
                    }
                }
                item_bigint = item_bigint % crt;

                // item_bigint = item_bigint % CRT;
                if &item_bigint % monkey.test_value == BigInt::from(0) {
                    destination = monkey.true_destination;
                } else {
                    destination = monkey.false_destination;
                }

                monkeys[m].items.remove(&item_monkey);
                monkeys
                    .get_mut(destination)
                    .unwrap()
                    .items
                    .insert(item_bigint.to_string());
            }
        }
        round_count += 1;
        if round_count % 1000 == 0 {
            dbg!(round_count);
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        for item in monkey.items.iter() {
            println!("{} {}", i, item)
        }
    }
    let mut active_monkeys = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    active_monkeys.sort();
    dbg!(&active_monkeys);
    let result2 = active_monkeys.into_iter().rev().take(2).product::<u64>();
    println!("Part2: {result2}");

    // Part 2 fix

    struct Monkey_Big2 {
        items: Vec<i64>,
        item_operations: String,
        item_value: i64,
        test_value: i64,
        true_destination: usize,
        false_destination: usize,
        count: u64,
    }

    let mut monkeys = Vec::<Monkey_Big2>::new();
    // initialize monkey vector
    let mut _items: Vec<i64> = Vec::new();
    let mut _operation: &str;
    let mut _item_operation: &str = "";
    let mut _item_value: i64 = 0;
    let mut _test_value: i64 = 0;
    let mut _true_destination: usize = 0;
    let mut _false_destination: usize = 0;
    for line in input.lines() {
        if line.contains("Starting") {
            _operation = line.split("items: ").last().unwrap();
            _items = _operation
                .split(", ")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        } else if line.contains("Operation") {
            _operation = line.split("new = ").last().unwrap();
            for word in _operation.split_whitespace() {
                if word.contains("+") {
                    _item_operation = "+";
                } else if word.contains("*") {
                    _item_operation = "*";
                } else if word.chars().any(|c| c.is_ascii_digit()) {
                    _item_value = word.parse::<i64>().unwrap();
                } else {
                    _item_value = 0;
                }
            }
        } else if line.contains("Test") {
            _operation = line.split("divisible by ").last().unwrap();
            _test_value = _operation.parse::<i64>().unwrap();
        } else if line.contains("true") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _true_destination = _operation.parse::<usize>().unwrap();
        } else if line.contains("false") {
            _operation = line.split("throw to monkey ").last().unwrap();
            _false_destination = _operation.parse::<usize>().unwrap();
        }
        if line.is_empty() {
            monkeys.push(Monkey_Big2 {
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
    let crt = monkeys.iter().map(|m| m.test_value).product::<i64>();
    while round_count < 10000 {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                if monkeys[m].items[i] == 0 {
                    continue;
                }
                monkeys[m].count += 1;
                let destination;
                let mut item: i64;

                let monkey = monkeys.get_mut(m).unwrap();
                item = monkey.items[i];
                if monkey.item_operations == "+" {
                    if monkey.item_value == 0 {
                        item += item;
                    } else {
                        item += monkey.item_value;
                    }
                } else if monkey.item_operations == "*" {
                    if monkey.item_value == 0 {
                        item *= item;
                    } else {
                        item *= monkey.item_value;
                    }
                }
                item = item % crt;
                if item % monkey.test_value == 0 {
                    destination = monkey.true_destination;
                } else {
                    destination = monkey.false_destination;
                }
                monkey.items[i] = 0;
                monkeys.get_mut(destination).unwrap().items.push(item);
            }
        }
        round_count += 1;
        if round_count % 1000 == 0 {
            dbg!(round_count);
        }
    }

    for monkey in monkeys.iter() {
        println!("{}", monkey.count)
    }

    let mut active_monkeys = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    active_monkeys.sort();
    dbg!(&active_monkeys);
    let result3 = active_monkeys.iter().rev().take(2).product::<u64>();
    println!("Part2_fix: {result3}");
}
