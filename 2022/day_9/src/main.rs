use std::{collections::HashSet, fs};

enum POSITION {
    SAME,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl POSITION {
    fn from_str(s: &str) -> POSITION {
        match s {
            "L" => POSITION::LEFT,
            "R" => POSITION::RIGHT,
            "U" => POSITION::UP,
            "D" => POSITION::DOWN,
            _ => POSITION::SAME,
        }
    }
}

fn correct_tail_movement(head: (i32, i32), tail: &mut (i32, i32)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dy.abs() > 1 || dx.abs() > 1 {
        if dy.abs() < dx.abs() {
            tail.1 = head.1;
            tail.0 = head.0 - dx.signum();
        } else if dy.abs() > dx.abs() {
            tail.0 = head.0;
            tail.1 = head.1 - dy.signum();
        } else {
            tail.0 = head.0 - dx.signum();
            tail.1 = head.1 - dy.signum();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();
    let mut step_collector = vec![HashSet::new(); 2];
    let segments_p1 = vec![(0, 0); 2];
    let segments_p2 = vec![(0, 0); 10];

    let mut segments_vec = vec![segments_p1, segments_p2];

    for line in input_lines {
        let mut instruction = line.split_whitespace();
        let curr_step_dir = POSITION::from_str(instruction.next().unwrap());
        let steps = instruction
            .next()
            .expect("No steps provided")
            .parse::<i32>()
            .expect("Unable to parse");

        for _ in 0..steps {
            let mut step_collector_index = 0;
            for segments in &mut segments_vec {
                match curr_step_dir {
                    POSITION::LEFT => segments[0].0 -= 1,
                    POSITION::RIGHT => segments[0].0 += 1,
                    POSITION::UP => segments[0].1 += 1,
                    POSITION::DOWN => segments[0].1 -= 1,
                    POSITION::SAME => (),
                }

                for i in 0..segments.len() - 1 {
                    correct_tail_movement(segments[i], &mut segments[i + 1]);
                }

                step_collector[step_collector_index].insert(segments[segments.len() - 1]);
                step_collector_index += 1;
            }
        }
    }

    // Part 1
    println!("Part 1: {}", step_collector[0].len());

    // Part 2
    println!("Part 2: {}", step_collector[1].len());
}
