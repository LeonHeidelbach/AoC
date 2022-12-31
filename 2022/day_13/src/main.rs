use std::{cmp, fs, iter::Peekable, str};

#[derive(PartialEq, Clone)]
enum EntityType {
    NONE,
    VALUE,
    LIST,
}

#[derive(PartialEq, Clone)]
struct Entity {
    e_type: EntityType,
    list: Option<Vec<Entity>>,
    value: Option<isize>,
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (
                Entity {
                    e_type: EntityType::VALUE,
                    value: Some(l),
                    ..
                },
                Entity {
                    e_type: EntityType::VALUE,
                    value: Some(r),
                    ..
                },
            ) => l.partial_cmp(r),
            (
                Entity {
                    e_type: EntityType::LIST,
                    list: Some(l),
                    ..
                },
                Entity {
                    e_type: EntityType::LIST,
                    list: Some(r),
                    ..
                },
            ) => l.partial_cmp(r),
            (
                Entity {
                    e_type: EntityType::VALUE,
                    value: Some(l),
                    ..
                },
                Entity {
                    e_type: EntityType::LIST,
                    list: Some(r),
                    ..
                },
            ) => {
                if r.is_empty() {
                    return Some(cmp::Ordering::Greater);
                }
                return vec![Entity {
                    e_type: EntityType::VALUE,
                    value: Some(*l),
                    list: None,
                }]
                .partial_cmp(r);
            }
            (
                Entity {
                    e_type: EntityType::LIST,
                    list: Some(l),
                    ..
                },
                Entity {
                    e_type: EntityType::VALUE,
                    value: Some(r),
                    ..
                },
            ) => {
                if l.is_empty() {
                    return Some(cmp::Ordering::Less);
                }
                return l.partial_cmp(&vec![Entity {
                    e_type: EntityType::VALUE,
                    value: Some(*r),
                    list: None,
                }]);
            }
            _ => panic!("Invalid comparison"),
        }
    }
}

impl Entity {
    fn new() -> Self {
        Self {
            e_type: EntityType::NONE,
            list: None,
            value: None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Entity {
                e_type: EntityType::VALUE,
                value: Some(l),
                ..
            } => l.to_string(),
            Entity {
                e_type: EntityType::LIST,
                list: Some(r),
                ..
            } => format!(
                "[{}]",
                r.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            _ => panic!("Invalid entity"),
        }
    }

    fn parse_inner_list(
        p: &mut Packet,
        chars: &mut Peekable<str::Chars>,
        entity: &mut Entity,
    ) -> bool {
        let mut acc_list = vec![];
        entity.e_type = EntityType::LIST;
        loop {
            let e = Entity::parse(p, chars);
            match e {
                Some(e) => {
                    acc_list.push(e);
                }
                None => {
                    break;
                }
            }
        }
        entity.list = Some(acc_list);
        return true;
    }

    fn parse_isize(c: char, chars: &mut Peekable<str::Chars>, entity: &mut Entity) -> bool {
        let mut acc_num = String::new();
        acc_num.push(c);
        entity.e_type = EntityType::VALUE;
        loop {
            match chars.peek() {
                Some('0'..='9') => acc_num.push(chars.next().unwrap()),
                None | Some(',') | Some(']') => {
                    entity.value = Some(acc_num.parse::<isize>().unwrap());
                    break;
                }
                _ => panic!(
                    "Unexpected input while parsing entity isize value: {:?}",
                    chars.peek()
                ),
            }
        }
        return true;
    }

    fn parse(p: &mut Packet, chars: &mut Peekable<str::Chars>) -> Option<Entity> {
        let mut entity = Entity::new();
        loop {
            let c = chars.next();
            let parse_break: bool = match c {
                Some('[') => Entity::parse_inner_list(p, chars, &mut entity),
                Some('0'..='9') => Entity::parse_isize(c.unwrap(), chars, &mut entity),
                Some(',') => false,
                Some(']') | None => true,
                _ => panic!("Unexpected input while parsing entity: {:?}", c.unwrap()),
            };
            if parse_break {
                break;
            }
        }
        if entity.e_type == EntityType::NONE {
            return None;
        }
        p.items += 1;
        return Some(entity);
    }
}

#[derive(PartialEq, Clone)]
struct Packet {
    items: usize,
    entities: Vec<Entity>,
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        for i in 0..self.items + 1 {
            if i >= self.entities.len() {
                return Some(cmp::Ordering::Less);
            } else if i >= other.entities.len() {
                return Some(cmp::Ordering::Greater);
            }
            let l = &self.entities[i];
            let r = &other.entities[i];
            if l > r {
                return Some(cmp::Ordering::Greater);
            } else if l < r {
                return Some(cmp::Ordering::Less);
            }
        }
        unreachable!("Comparison should have returned before this point");
    }
}

impl Packet {
    fn new() -> Self {
        Self {
            items: 0,
            entities: vec![],
        }
    }

    fn from_slice(s: &str) -> Self {
        let mut p = Packet::new();
        p.parse_packet(s);
        return p;
    }

    fn parse_packet(&mut self, line: &str) {
        let mut line = line[1..line.len() - 1].chars().peekable();
        while let Some(entity) = Entity::parse(self, &mut line) {
            self.entities.push(entity);
        }
    }
}

struct Pair {
    p1: Packet,
    p2: Packet,
    correct_order: bool,
}

impl Pair {
    fn new() -> Self {
        Self {
            p1: Packet::new(),
            p2: Packet::new(),
            correct_order: true,
        }
    }

    fn parse_pair(&mut self, p1_str: &str, p2_str: &str) {
        self.p1.parse_packet(p1_str);
        self.p2.parse_packet(p2_str);
    }

    fn validate_order(&mut self) {
        if self.p1.partial_cmp(&self.p2) == Some(cmp::Ordering::Greater) {
            self.correct_order = false;
        }
    }
}

fn print_packet_list(packets: &Vec<Packet>) {
    packets.iter().enumerate().for_each(|(i, p)| {
        println!(
            "{}: {:?}",
            format!(
                "{:width$}",
                i + 1,
                width = (packets.len() as f64).log10() as usize + 1
            ),
            p.entities
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
        )
    });
}

fn main() {
    let input: String = fs::read_to_string("input_p1").expect("Unable to read file");
    let mut input_lines: str::Lines = input.lines();
    let mut pairs: Vec<Pair> = vec![];

    loop {
        let p1_str: Option<&str> = input_lines.next();
        if p1_str.is_none() {
            break;
        } else if p1_str.unwrap().is_empty() {
            continue;
        }
        let p2_str: Option<&str> = input_lines.next();
        let mut new_pair: Pair = Pair::new();
        new_pair.parse_pair(p1_str.unwrap(), p2_str.unwrap());
        new_pair.validate_order();
        pairs.push(new_pair);
    }

    // Part 1
    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.correct_order)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1: {:?}", sum);

    // Part 2
    let d_2 = Packet::from_slice("[[2]]");
    let d_6 = Packet::from_slice("[[6]]");

    let mut packets: Vec<Packet> = pairs
        .iter()
        .flat_map(|pair| vec![pair.p1.clone(), pair.p2.clone()])
        .collect();

    packets.push(d_2.clone());
    packets.push(d_6.clone());
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let i_2 = packets.iter().position(|p| p == &d_2).unwrap();
    let i_6 = packets.iter().position(|p| p == &d_6).unwrap();

    println!(
        "Part 2: {:?} * {:?} = {:?}",
        i_2 + 1,
        i_6 + 1,
        (i_2 + 1) * (i_6 + 1)
    );

    // Optional: To print the ordered list of packets
    print_packet_list(&packets);
}
