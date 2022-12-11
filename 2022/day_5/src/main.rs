use std::fs;

const CRATE_WIDTH: u32 = 3;
const CRATE_PADDING: u32 = 1;

#[derive(Clone)]
struct Crate {
    id: String,
}

impl Crate {
    fn new(id: String) -> Self {
        Self { id }
    }
}

fn parse_crate_schema(
    stacks: &mut Vec<Vec<Crate>>,
    stack_count: &mut u32,
    input_lines: &mut std::str::Lines,
) {
    loop {
        let line = input_lines.next();
        if line.is_none() || line.unwrap().is_empty() {
            break;
        };
        parse_crate_schema_line(stacks, stack_count, line.unwrap());
    }
}

fn parse_crate_schema_line(stacks: &mut Vec<Vec<Crate>>, stack_count: &mut u32, line: &str) {
    let crate_id: &mut String = &mut String::new();
    let mut current_stack: &mut Vec<Crate> = &mut stacks[0];
    let mut stack_index: u32 = 0;
    let mut new_crate: bool = false;

    for (index, c) in line.chars().enumerate() {
        if index as u32 % (CRATE_WIDTH + CRATE_PADDING) == 0 {
            stack_index += 1;
            if stack_count < &mut stack_index {
                stacks.push(vec![]);
                *stack_count += 1;
            }
            current_stack = &mut stacks[stack_index as usize - 1];
        }

        match c {
            '[' => new_crate = true,
            ']' => {
                new_crate = false;
                current_stack.push(Crate::new(crate_id.to_string()));
                crate_id.clear();
            }
            ' ' => continue,
            _ => {
                if new_crate {
                    crate_id.push(c);
                }
            }
        }
    }
}

fn parse_instructions(stacks: &mut Vec<Vec<Crate>>, input_lines: &mut std::str::Lines, model: u32) {
    loop {
        let line = input_lines.next();
        if line.is_none() {
            break;
        };
        let instruction: &mut std::str::SplitWhitespace =
            &mut line.unwrap().split_whitespace().into_iter();
        match instruction.next().unwrap() {
            "move" => {
                move_crates(stacks, instruction, model);
            }
            _ => continue,
        }
    }
}

fn move_crates(
    stacks: &mut Vec<Vec<Crate>>,
    instruction: &mut std::str::SplitWhitespace,
    model: u32,
) {
    let move_amount = instruction.next().unwrap().parse::<usize>().unwrap();
    let move_from_index = instruction
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;
    let move_to_index = instruction
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let move_items: Vec<Crate> = if model == 9000 {
        stacks
            .get_mut(move_from_index)
            .unwrap()
            .drain(..move_amount)
            .into_iter()
            .rev()
            .collect()
    } else if model == 9001 {
        stacks
            .get_mut(move_from_index)
            .unwrap()
            .drain(..move_amount)
            .into_iter()
            .collect()
    } else {
        panic!("Invalid model number")
    };
    stacks
        .get_mut(move_to_index)
        .unwrap()
        .splice(0..0, move_items);
}

fn main() {
    let mut stacks_9000: Vec<Vec<Crate>> = vec![vec![]];
    let mut stacks_9001: Vec<Vec<Crate>>;
    let mut stack_count: u32 = 1;
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let mut input_lines = input.lines();

    parse_crate_schema(&mut stacks_9000, &mut stack_count, &mut input_lines);
    stacks_9001 = stacks_9000.clone();

    let mut input_lines_clone = input_lines.clone();

    parse_instructions(&mut stacks_9000, &mut input_lines, 9000);
    parse_instructions(&mut stacks_9001, &mut input_lines_clone, 9001);

    // Part 1
    println!(
        "Part 1: {:?}",
        stacks_9000
            .iter()
            .flat_map(|s| s.iter().next())
            .map(|c| c.id.to_string())
            .collect::<String>()
    );

    // Part 2
    println!(
        "Part 2: {:?}",
        stacks_9001
            .iter()
            .flat_map(|s| s.iter().next())
            .map(|c| c.id.to_string())
            .collect::<String>()
    );
}
