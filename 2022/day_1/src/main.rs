use std::fs;

struct Elf {
    items: Vec<usize>,
}

impl Elf {
    pub fn calorie_sum(&self) -> usize {
        self.items.iter().sum()
    }
}

fn main() {
    let mut elves: Vec<Elf> = vec![Elf { items: vec![] }];
    let mut curr_elf: &mut Elf = elves.last_mut().unwrap();
    let input: String = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines: Vec<&str> = input.lines().collect();

    for line in input_lines {
        if line.is_empty() {
            elves.push(Elf { items: vec![] });
            curr_elf = elves.last_mut().unwrap();
            continue;
        }
        curr_elf.items.push(line.parse::<usize>().unwrap());
    }

    // Part 1
    println!(
        "Part 1: {}",
        elves.iter().map(Elf::calorie_sum).max().unwrap()
    );

    // Part 2
    let mut elves_sorted = elves.iter().map(Elf::calorie_sum).collect::<Vec<usize>>();
    elves_sorted.sort();

    println!(
        "Part 2: {:?}",
        elves_sorted.iter().rev().take(3).sum::<usize>()
    );
}
