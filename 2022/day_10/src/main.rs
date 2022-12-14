use std::fs;

#[derive(Default)]
struct Instruction {
    cmd: String,
    param: Option<isize>,
}

struct Screen {
    canvas: Vec<Vec<char>>,
    sprite_pos: (usize, usize),
    current_pixel: (usize, usize),
}

impl Screen {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    const SPRITE_WIDTH: usize = 3;

    fn new() -> Self {
        Self {
            canvas: vec![vec!['.'; Screen::WIDTH]; Screen::HEIGHT],
            sprite_pos: (0, 0),
            current_pixel: (0, 0),
        }
    }

    fn draw_current_pixel(&mut self) {
        if self.current_pixel.1 >= self.sprite_pos.1
            && self.current_pixel.1 < self.sprite_pos.1 + Screen::SPRITE_WIDTH
        {
            self.canvas[self.current_pixel.0][self.current_pixel.1] = '#';
        }
        if self.current_pixel.1 > 0 && self.current_pixel.1 % 39 == 0 {
            self.current_pixel.1 = 0;
            self.current_pixel.0 += 1;
        } else {
            self.current_pixel.1 += 1;
        }
    }

    fn update_sprite_pos(&mut self, x: isize) {
        self.sprite_pos.1 = (x - 1).clamp(0, 39) as usize;
    }

    fn print_screen(&self) {
        self.canvas.iter().for_each(|row| {
            row.iter().for_each(|pixel| print!("{}", pixel));
            println!();
        });
    }
}

struct CPU {
    x: isize,
    wait: isize,
    cycles: isize,
    signal_strength: isize,
    instructions: Vec<Instruction>,
    screen: Screen,
}

impl CPU {
    fn new() -> Self {
        return Self {
            x: 1,
            wait: 0,
            cycles: 0,
            signal_strength: 0,
            instructions: vec![],
            screen: Screen::new(),
        };
    }

    fn push_instruction(&mut self, ins_line: &str) {
        let values: Vec<&str> = ins_line.split_whitespace().collect();
        self.instructions.push(Instruction {
            cmd: values[0].to_string(),
            param: if values.len() > 1 {
                values[1].to_string().parse::<isize>().ok()
            } else {
                None
            },
        })
    }

    fn execute_instruction(&mut self) {
        let current_instruction = self.instructions.last().expect("No stored instructions");

        match current_instruction.cmd.as_str() {
            "addx" => {
                self.wait = 2;
                while self.wait > 0 {
                    self.cycles += 1;
                    self.screen.draw_current_pixel();
                    self.signal_strength += self.calculate_signal_strength();
                    self.wait -= 1;
                }

                self.x += current_instruction
                    .param
                    .expect("Param should have a value in 'addx' instruction.");

                self.screen.update_sprite_pos(self.x);
            }
            "noop" => {
                self.cycles += 1;
                self.signal_strength += self.calculate_signal_strength();
                self.screen.draw_current_pixel();
            }
            _ => panic!("Unknown instruction {}", current_instruction.cmd),
        }
    }

    fn calculate_signal_strength(&self) -> isize {
        if self.cycles == 20 || (self.cycles - 20) % 40 == 0 {
            return self.x * self.cycles;
        }
        return 0;
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();
    let mut cpu = CPU::new();

    for line in input_lines {
        cpu.push_instruction(line);
        cpu.execute_instruction();
    }

    // Part 1
    println!("Part 1: {:?}", cpu.signal_strength);

    // Part 2
    println!("Part 2:");
    cpu.screen.print_screen();
}
