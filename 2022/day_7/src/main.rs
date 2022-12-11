use std::{collections::HashMap, fs};

fn main() {
    let mut curr_dir: Vec<&str> = vec!["/"];
    let mut dirs_raw: HashMap<String, u32> = HashMap::new();
    let input: String = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines: std::str::Lines = input.lines();

    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir: &str = line.trim_start_matches("$ cd ");
            if dir == ".." {
                curr_dir.pop();
                continue;
            } else if dir == "/" {
                curr_dir.clear();
            }
            curr_dir.push(dir.clone());
            dirs_raw.insert(curr_dir.join("/").to_string(), 0);
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let dir: String = curr_dir.join("/") + "/" + line.trim_start_matches("dir ");
            if !dirs_raw.contains_key(&dir) {
                dirs_raw.insert(dir, 0);
            }
        } else {
            let file: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            let file_size: u32 = file[0].parse::<u32>().unwrap();
            let curr_dir_str: String = curr_dir.join("/");
            dirs_raw.insert(
                curr_dir_str.clone(),
                dirs_raw.get(&curr_dir_str.clone()).unwrap() + file_size,
            );
        }
    }

    let mut dirs_evaled: HashMap<String, u32> = dirs_raw.clone();

    for (k_1, v_1) in &dirs_raw {
        for (k_2, _) in &dirs_raw {
            if k_1 == k_2 {
                continue;
            };
            if k_1.contains(k_2) {
                dirs_evaled.insert(k_2.to_string(), dirs_evaled.get(k_2).unwrap() + v_1);
            };
        }
    }

    let sum: u32 = dirs_evaled
        .iter()
        .filter(|item| item.1 <= &100000)
        .map(|item| item.1)
        .sum::<u32>();

    // Part 1
    println!("Part 1: {:?}", sum);

    // Part 2
    let mut dirs_evaled_sorted: Vec<(&String, &u32)> = dirs_evaled.iter().collect();
    dirs_evaled_sorted.sort_by(|a, b| a.1.cmp(b.1));

    let root_dir_size: i32 = *dirs_evaled_sorted
        .iter()
        .find(|item| item.0 == &"/")
        .unwrap()
        .1 as i32;
    let available_storage: i32 = 70000000 - root_dir_size;

    println!(
        "Part 2: {:?}",
        dirs_evaled_sorted
            .iter()
            .find(|item| available_storage + *item.1 as i32 >= 30000000)
            .unwrap()
            .1
    );
}
