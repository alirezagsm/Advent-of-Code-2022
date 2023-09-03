use std::collections::BTreeMap;

struct File {
    name: String,
    size: usize,
    parent: String,
}

fn main() {
    let input = include_str!("input2.txt");
    let mut current_dir = " /".to_string();
    let mut next_dir: String;
    // let mut dirname = " /".to_string();
    let f = File {
        name: current_dir.to_string(),
        size: 0,
        parent: "_".to_string(),
    };
    let mut filemap: BTreeMap<String, File> = BTreeMap::new();
    filemap.insert(f.name.clone(), f);

    for (mut _n, line) in input.lines().enumerate() {
        if line.contains("$ cd") {
            // println!("{n}---{line}");
            next_dir = line.split("cd").collect::<Vec<&str>>()[1].to_string();
            if next_dir == " .." {
                next_dir = get_prev_dir(&current_dir);
            } else {
                next_dir = get_dirname(&current_dir, &next_dir);
            }
            // println!("{current_dir} to {next_dir}");
            if let Some(_file) = filemap.get(&next_dir) {
                // println!("{}: exists!", next_dir);
                current_dir = next_dir.clone();
            } else {
                filemap.insert(
                    next_dir.clone(),
                    File {
                        name: next_dir.clone(),
                        size: 0,
                        parent: current_dir.clone(),
                    },
                );
                // println!("{}: added to filemap by cd", next_dir);
                current_dir = next_dir.clone();
            }
        }
        // else if line.contains("dir") {
        //     println!("{n}---{line}");
        //     next_dir = line.split("dir").collect::<Vec<&str>>()[1].to_string();
        //     dirname = get_dirname(&current_dir, &next_dir);
        //     if let Some(_file) = filemap.get(&dirname) {
        //         println!("{}: exists!", dirname);
        //     } else {
        //         dirname = get_dirname(&current_dir, &next_dir);
        //         filemap.insert(
        //             dirname.clone(),
        //             File {
        //                 name: dirname.clone(),
        //                 size: 0,
        //                 parent: current_dir.clone(),
        //             },
        //         );
        //         println!("{}: added to filemap by dir", dirname);
        //     }
        // }
        if line.chars().any(|c| c.is_numeric()) {
            // println!("{n}---{line}");
            let size_str = line.split(" ").collect::<Vec<&str>>()[0];
            let size = size_str.trim().parse::<usize>().unwrap();
            filemap.get_mut(&current_dir).unwrap().size += size;
            // println!("{}: updated size", dirname);
        }
    }

    let keys = filemap.keys().map(|k| k.to_string()).collect::<Vec<_>>();

    // println!("\n\nFilesystem:");
    // for (i, (key, _value)) in filemap.iter().enumerate() {
    //     println!("{i}: {key}");
    // }
    // println!("");
    // println!("\n\nFilesystem:");
    // for (key, value) in filemap.iter() {
    //     println!("{} {} {} ", key, value.size, value.parent);
    // }

    // walk the file system and update sizes
    for key in keys.iter().rev() {
        let value = filemap.get(key as &str).unwrap();
        if value.parent != "_" {
            // println!(
            //     "parent:{}, current:{}, size:{}",
            //     &value.parent, &value.name, &value.size
            // );
            filemap.get_mut(&value.parent.clone()).unwrap().size += value.size;
        }
    }

    println!("\n\nFilesystem:");
    for (key, value) in filemap.iter() {
        println!("{} {} {} ", key, value.size, value.parent);
    }

    let result = filemap
        .values()
        .filter(|v| v.size <= 100_000)
        .map(|v| v.size)
        .sum::<usize>();

    println!("Part1: {}", result);

    // Part 2
    let total = 70000000;
    let min_free = 30000000;
    let to_free = min_free - (total - filemap.get(" /").unwrap().size);
    
    println!("to_free: {}", to_free);
    let candidates = filemap
        .values()
        .into_iter()
        .filter(|v| v.size >= to_free)
        .collect::<Vec<_>>();
    let result2 = candidates.iter().map(|v| v.size).min().unwrap();

    println!("Part2: {}", result2);
}

fn get_dirname(current_dir: &str, next_dir: &str) -> String {
    if next_dir == " /" {
        " /".to_string()
    } else {
        current_dir.to_string() + "=>" + next_dir
    }
}
fn get_prev_dir(current_dir: &str) -> String {
    let _dir = current_dir.split("=>").collect::<Vec<&str>>();
    let _dir = _dir[.._dir.len() - 1].join("=>");
    _dir
}
