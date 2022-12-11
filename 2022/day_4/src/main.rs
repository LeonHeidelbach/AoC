use std::fs;

enum State {
    CONTAINS,
    INTERSECTS,
    UNRELATED,
}

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn new(start: u32, end: u32) -> Self {
        return Self { start, end };
    }

    fn compare(&self, other: &Section) -> State {
        return match (self.start, self.end, other.start, other.end) {
            (s1, e1, s2, e2) if s1 <= s2 && e1 >= e2 => State::CONTAINS,
            (s1, e1, s2, e2) if s1 >= s2 && e1 <= e2 => State::CONTAINS,
            (s1, e1, s2, _) if s1 <= s2 && e1 >= s2 => State::INTERSECTS,
            (s1, e1, _, e2) if s1 <= e2 && e1 >= e2 => State::INTERSECTS,
            _ => State::UNRELATED,
        };
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();
    let mut contains_counter = 0;
    let mut intersect_counter = 0;

    for line in input_lines {
        let mut pair_vec = vec![];
        let pairs = line.split(",");

        for pair in pairs {
            let bounds = pair.split("-");
            let sec: Section = Section::new(
                bounds.clone().nth(0).unwrap().parse::<u32>().unwrap(),
                bounds.clone().nth(1).unwrap().parse::<u32>().unwrap(),
            );
            pair_vec.push(sec);
        }
        contains_counter += match pair_vec[0].compare(&pair_vec[1]) {
            State::CONTAINS => 1,
            _ => 0,
        };
        intersect_counter += match pair_vec[0].compare(&pair_vec[1]) {
            State::INTERSECTS | State::CONTAINS => 1,
            _ => 0,
        }
    }

    // Part 1
    println!("Part 1: {:?}", contains_counter);

    // Part 2
    println!("Part 2: {:?}", intersect_counter);
}
