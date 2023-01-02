use primitive_types::U512;
use std::fs;

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn parse(input: &str) -> Self {
        let mut parts = input.split(',');
        return Self::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        );
    }
}

struct Line {
    points: Vec<Point>,
    x_low: u32,
    y_low: u32,
    x_high: u32,
    y_high: u32,
}

impl Line {
    fn new(points: Vec<Point>, x_low: u32, y_low: u32, x_high: u32, y_high: u32) -> Self {
        Self {
            points,
            x_low,
            y_low,
            x_high,
            y_high,
        }
    }

    fn parse(input: &str) -> Self {
        let mut x_low = u32::MAX;
        let mut y_low = u32::MAX;
        let mut x_high = 0;
        let mut y_high = 0;

        let points = input
            .split(" -> ")
            .map(|x| {
                let p = Point::parse(x);
                if p.x < x_low {
                    x_low = p.x;
                }
                if p.y < y_low {
                    y_low = p.y;
                }
                if p.x > x_high {
                    x_high = p.x;
                }
                if p.y > y_high {
                    y_high = p.y;
                }
                return p;
            })
            .collect::<Vec<Point>>();
        return Self::new(points, x_low, y_low, x_high, y_high);
    }
}

struct Cave {
    map: Vec<U512>,
    sand: Vec<U512>,
    source: Point,
    sand_amount: u32,
    x_low: u32,
    y_low: u32,
    x_high: u32,
    y_high: u32,
    x_diff: u32,
    y_diff: u32,
}

impl Cave {
    fn new() -> Self {
        Self {
            map: vec![],
            sand: vec![],
            source: Point::new(0, 0),
            sand_amount: 0,
            x_low: u32::MAX,
            y_low: u32::MAX,
            x_high: 0,
            y_high: 0,
            x_diff: 0,
            y_diff: 0,
        }
    }

    fn from_lines(source: Point, lines: &Vec<Line>) -> Self {
        let mut cave = Cave::new();
        cave.source = source;
        cave.calculate_extremes(lines);
        cave.draw_lines(lines);
        return cave;
    }

    fn calculate_extremes(&mut self, lines: &Vec<Line>) {
        lines.iter().for_each(|l| {
            if l.x_low < self.x_low {
                self.x_low = l.x_low;
            }
            if l.y_low < self.y_low {
                self.y_low = l.y_low;
            }
            if l.x_high > self.x_high {
                self.x_high = l.x_high;
            }
            if l.y_high > self.y_high {
                self.y_high = l.y_high;
            }
        });
        self.x_diff = self.x_high - self.x_low;
        self.y_diff = self.y_high - self.y_low;
        self.map = vec![U512::from(0); (self.y_high + 1) as usize];
        assert!(
            self.x_diff <= 512,
            "Map width {:?} too large for U512",
            self.x_diff
        );
    }

    fn draw_lines(&mut self, lines: &Vec<Line>) {
        lines
            .iter()
            .flat_map(|l| l.points.iter().zip(l.points.iter().skip(1)))
            .for_each(|(p, p_next)| {
                if p.x == p_next.x {
                    let y_low = p.y.min(p_next.y);
                    let y_high = p.y.max(p_next.y);
                    for y in y_low..=y_high {
                        self.map[y as usize] |= U512::from(1) << (p.x - self.x_low);
                    }
                } else {
                    let x_low = p.x.min(p_next.x);
                    let x_high = p.x.max(p_next.x);
                    for x in x_low..=x_high {
                        self.map[p.y as usize] |= U512::from(1) << (x - self.x_low);
                    }
                }
            });
        self.sand = self.map.clone();
    }

    fn add_plane(&mut self, y_offset: u32) {
        (1..=y_offset - 1).for_each(|_| {
            self.map.push(U512::from(0));
            self.sand.push(U512::from(0));
        });
        self.map.push(U512::MAX);
        self.sand.push(U512::MAX);
    }

    fn shift_map(&mut self, offset: u32) {
        self.map = self.map.iter().map(|x| x << offset).collect::<Vec<U512>>();
        self.sand = self.sand.iter().map(|x| x << offset).collect::<Vec<U512>>();
        self.map.last_mut().map(|x| *x = U512::MAX);
        self.sand.last_mut().map(|x| *x = U512::MAX);
        self.x_low -= offset;
        self.x_high -= offset;
        self.x_diff += offset;
    }

    fn simulate_sand_from_source(&mut self) -> u32 {
        'a: loop {
            let mut drop_mask: U512 = U512::from(1) << (self.source.x - self.x_low);
            let mut sand_set = false;
            for i in 0..self.sand.len() - 1 {
                if !sand_set
                    && self.sand[i] & drop_mask == U512::from(0)
                    && self.sand[i + 1] & drop_mask == drop_mask
                {
                    let mut l_drop_offset: U512 = drop_mask >> 1;
                    if l_drop_offset == U512::from(0) {
                        if self.sand.last() == Some(&U512::MAX) {
                            self.shift_map(1);
                            l_drop_offset = U512::from(1);
                            drop_mask <<= 1;
                        } else {
                            continue;
                        }
                    }
                    let r_drop_offset: U512 = drop_mask << 1;
                    if self.sand[i + 1] & l_drop_offset == l_drop_offset {
                        if self.sand.last() == Some(&U512::MAX)
                            || r_drop_offset < U512::from(1) << (self.x_diff + 1)
                        {
                            if r_drop_offset > U512::from(1) << self.x_diff {
                                self.x_diff += 1;
                            }
                            if self.sand[i + 1] & r_drop_offset == r_drop_offset {
                                self.sand[i] |= drop_mask;
                                sand_set = true;
                                self.sand_amount += 1;
                                if i == 0 {
                                    break 'a;
                                }
                                break;
                            } else {
                                drop_mask <<= 1;
                            }
                        }
                    } else {
                        drop_mask >>= 1;
                    }
                }
            }
            if !sand_set {
                break;
            }
        }
        return self.sand_amount;
    }

    fn print_map(&self) {
        self.map
            .iter()
            .zip(self.sand.iter())
            .enumerate()
            .for_each(|(i, (map_row, sand_row))| {
                let sand = *map_row ^ *sand_row;
                let mut line = String::new();
                (0..=self.x_diff)
                    .map(|x| {
                        if self.source.x == x + self.x_low && self.source.y == i as u32 {
                            return '+';
                        } else if sand & (U512::from(1) << x) == U512::from(1) << x {
                            'o'
                        } else if *map_row & (U512::from(1) << x) == U512::from(1) << x {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .for_each(|c| line.push(c));
                println!(
                    "{:width$} {}",
                    i,
                    line,
                    width = self.y_diff.to_string().len() + 1
                );
            });
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let lines = input.lines().map(|x| Line::parse(x)).collect::<Vec<Line>>();
    let mut cave = Cave::from_lines(Point::new(500, 0), &lines);

    // Part 1
    println!("Part 1: {}", cave.simulate_sand_from_source());

    // Part 2
    cave.add_plane(2);
    println!("Part 2: {}", cave.simulate_sand_from_source());

    // Optional: Print the map
    cave.print_map();
}
