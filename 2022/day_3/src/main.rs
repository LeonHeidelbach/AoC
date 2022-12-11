use std::{collections::HashSet, fs, ops::Add};

struct Backpack {
    contents: String,
    first_comp: String,
    second_comp: String,
}

impl Backpack {
    fn new(contents: &String) -> Backpack {
        let slices: (&str, &str) = contents.split_at(contents.len() / 2);
        return Backpack {
            contents: contents.to_string(),
            first_comp: slices.0.to_string(),
            second_comp: slices.1.to_string(),
        };
    }

    fn get_duplicate_items(&self) -> HashSet<u32> {
        let mut dups: HashSet<u32> = HashSet::new();
        for c in self.first_comp.chars() {
            if self.second_comp.contains(c) {
                dups.insert(c as u32 - (if c.is_uppercase() { 38 } else { 96 }));
            }
        }
        return dups;
    }

    fn find_group_badge_name(&self, bp_2: &Backpack, bp_3: &Backpack) -> u32 {
        for c in self.contents.chars() {
            if bp_2.contents.contains(c) && bp_3.contents.contains(c) {
                return c as u32 - (if c.is_uppercase() { 38 } else { 96 });
            }
        }
        panic!("No badge name found");
    }
}

fn main() {
    let mut bps: Vec<Backpack> = vec![];
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();

    for line in input_lines {
        let bp = Backpack::new(&line.to_string());
        bps.push(bp);
    }

    // Part 1
    println!(
        "Part 1: {:?}",
        bps.iter()
            .flat_map(Backpack::get_duplicate_items)
            .reduce(Add::add)
            .unwrap()
    );

    // Part 2
    let mut sum = 0;
    let mut bps_iter = bps.iter();

    loop {
        let bp_1 = bps_iter.next();
        let bp_2 = bps_iter.next();
        let bp_3 = bps_iter.next();
        if bp_1.is_none() || bp_2.is_none() || bp_3.is_none() {
            break;
        };
        sum += bp_1
            .unwrap()
            .find_group_badge_name(bp_2.unwrap(), bp_3.unwrap());
    }

    println!("Part 2: {:?}", sum);
}
