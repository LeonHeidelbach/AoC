use std::fs;

struct Monkey {
    starting_items: Vec<isize>,
    throw_to_true: isize,
    throw_to_false: isize,
    inspect_counter: isize,
    operation_fn: Option<Box<dyn Fn(isize, isize) -> isize>>,
    operation_val: isize,
    test_fn: Option<Box<dyn Fn(isize, isize) -> bool>>,
    test_val: isize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            starting_items: vec![],
            throw_to_true: -1,
            throw_to_false: -1,
            inspect_counter: 0,
            operation_fn: None,
            operation_val: 0,
            test_fn: None,
            test_val: -1,
        }
    }

    fn parse_starting_items(&mut self, input: String) {
        self.starting_items = input
            .trim_start_matches("  Starting items: ")
            .to_string()
            .split(", ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
    }

    fn parse_operation(&mut self, input: String) {
        let mut expr_mask: u32 = 0;
        let expression_items = input
            .trim_start_matches("  Operation: new = ")
            .split_whitespace();

        expression_items.for_each(|el| match el {
            "old" => expr_mask |= 1 << 0,
            "*" => expr_mask |= 1 << 1,
            "+" => expr_mask |= 1 << 2,
            _ => {
                self.operation_val = el.parse::<isize>().expect("Element should be parsable.");
                expr_mask |= 1 << 3
            }
        });

        self.operation_fn = match expr_mask {
            0b0011 => Some(Box::new(|old, _| old * old)),
            0b0101 => Some(Box::new(|old, _| old + old)),
            0b1011 => Some(Box::new(|old, new| old * new)),
            0b1101 => Some(Box::new(|old, new| old + new)),
            _ => panic!("Unknown operation mask: {}.", expr_mask),
        };
    }

    fn parse_test_condition(&mut self, input: String) {
        let expression_items = input
            .trim_start_matches("  Operation: new = ")
            .split_whitespace();
        self.test_val = expression_items
            .last()
            .expect("Test value should exist.")
            .parse::<isize>()
            .expect("Test value should be parsable.");
        self.test_fn = Some(Box::new(|x, y| x % y == 0));
    }

    fn parse_test_condition_true(&mut self, input: String) {
        self.throw_to_true = input
            .trim_start_matches("    If true: throw to monkey ")
            .parse::<isize>()
            .expect("Expected to parse an isize for test_condition_true");
    }

    fn parse_test_condition_false(&mut self, input: String) {
        self.throw_to_false = input
            .trim_start_matches("    If false: throw to monkey ")
            .parse::<isize>()
            .expect("Expected to parse an isize for test_condition_false");
    }
}

struct Game {
    monkeys: Vec<Box<Monkey>>,
}

impl Game {
    fn new() -> Self {
        Self { monkeys: vec![] }
    }

    fn add_monkey(&mut self) -> &mut Monkey {
        self.monkeys.push(Box::new(Monkey::new()));
        return self.monkeys.last_mut().unwrap();
    }

    fn parse_input(&mut self, input_lines: &mut std::str::Lines) {
        while let Some(line) = input_lines.next() {
            if line.starts_with("Monkey") {
                let current_monkey: &mut Monkey = self.add_monkey();
                current_monkey.parse_starting_items(input_lines.next().unwrap().to_string());
                current_monkey.parse_operation(input_lines.next().unwrap().to_string());
                current_monkey.parse_test_condition(input_lines.next().unwrap().to_string());
                current_monkey.parse_test_condition_true(input_lines.next().unwrap().to_string());
                current_monkey.parse_test_condition_false(input_lines.next().unwrap().to_string());
            }
        }
    }

    fn play(&mut self, rounds: isize, limit: Option<isize>) {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                let mut move_to = vec![];
                for j in 0..self.monkeys[i].starting_items.len() {
                    let mut current_worry = self.monkeys[i]
                        .operation_fn
                        .as_ref()
                        .expect("Operation should evaluate expression.")(
                        self.monkeys[i].starting_items[j],
                        self.monkeys[i].operation_val,
                    );

                    if limit.is_none() {
                        current_worry = (current_worry / 3) as isize;
                        self.monkeys[i].starting_items[j] = current_worry;
                    } else {
                        current_worry = current_worry % limit.unwrap();
                        self.monkeys[i].starting_items[j] = current_worry;
                    }

                    if self.monkeys[i]
                        .test_fn
                        .as_ref()
                        .expect("Test should return evaluation result.")(
                        current_worry,
                        self.monkeys[i].test_val,
                    ) {
                        move_to.push(self.monkeys[i].throw_to_true);
                    } else {
                        move_to.push(self.monkeys[i].throw_to_false);
                    }

                    self.monkeys[i].inspect_counter += 1;
                }

                move_to.iter().for_each(|x| {
                    let val = self.monkeys[i].starting_items.remove(0);
                    self.monkeys[*x as usize].starting_items.push(val);
                });
            }
        }
    }

    fn get_monkey_business_score(&mut self, range: usize) -> isize {
        self.monkeys
            .sort_by(|a, b| a.inspect_counter.cmp(&b.inspect_counter));

        return self
            .monkeys
            .iter()
            .rev()
            .take(range)
            .fold(1, |acc, x| acc * x.inspect_counter)
            .clone();
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let mut input_lines_1: std::str::Lines = input.lines().into_iter();
    let mut input_lines_2: std::str::Lines = input.lines().into_iter();
    let mut game_1: Game = Game::new();
    let mut game_2: Game = Game::new();

    // Part 1
    game_1.parse_input(&mut input_lines_1);
    game_1.play(20, None);

    println!("Part 1: {:?}", game_1.get_monkey_business_score(2));

    // Part 2
    game_2.parse_input(&mut input_lines_2);
    let limit = game_2.monkeys.iter().fold(1, |acc, x| acc * x.test_val);
    game_2.play(10_000, Some(limit));

    println!("Part 2: {:?}", game_2.get_monkey_business_score(2));
}
