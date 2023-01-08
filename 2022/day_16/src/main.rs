use std::{collections::HashMap, fs};

struct Valve {
    name: String,
    flow_rate: isize,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, flow_rate: isize, tunnels: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            tunnels,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut parts = input.split("; ");
        let mut f_parts = parts
            .next()
            .unwrap()
            .strip_prefix("Valve ")
            .unwrap()
            .split(" has flow rate=");
        return Self::new(
            f_parts.next().unwrap().to_string(),
            f_parts.next().unwrap().parse::<isize>().unwrap(),
            parts
                .next()
                .unwrap()
                .trim_matches(|c: char| c.is_lowercase() || c.is_whitespace())
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
        );
    }
}

struct Cave {
    minutes: isize,
    valves: Vec<Valve>,
    v_index: HashMap<String, usize>,
    nz_frate_v_index: HashMap<String, usize>,
    distance_field: HashMap<String, HashMap<String, isize>>,
    start_valve: String,
}

impl Cave {
    fn new(minutes: isize, valves: Vec<Valve>, start_valve: String) -> Self {
        Self {
            minutes,
            valves,
            v_index: HashMap::new(),
            nz_frate_v_index: HashMap::new(),
            start_valve,
            distance_field: HashMap::new(),
        }
    }

    fn calculate_lookup_tables(&mut self) -> &mut Self {
        self.valves
            .iter()
            .filter(|v| v.flow_rate > 0)
            .enumerate()
            .for_each(|(i, v)| {
                self.nz_frate_v_index.insert(v.name.clone(), i);
            });

        self.valves.iter().enumerate().for_each(|(i, v)| {
            self.v_index.insert(v.name.clone(), i);
        });

        return self;
    }

    fn calculate_distance_field(&mut self) -> &mut Self {
        self.valves.iter().for_each(|v| {
            let mut distance_map = HashMap::new();
            let mut visited = vec![];
            let mut queue = vec![v.name.clone()];
            let mut distance = 0;
            while !queue.is_empty() {
                let mut new_queue = vec![];
                queue.iter().for_each(|name| {
                    if !visited.contains(name) {
                        visited.push(name.clone());
                        if &v.name != name && self.valves[self.v_index[name]].flow_rate > 0 {
                            distance_map.insert(name.clone(), distance);
                        }
                        new_queue.append(&mut self.valves[self.v_index[name]].tunnels.clone());
                    }
                });
                queue = new_queue;
                distance += 1;
            }
            if v.name == self.start_valve || self.valves[self.v_index[&v.name]].flow_rate > 0 {
                self.distance_field.insert(v.name.clone(), distance_map);
            }
        });
        return self;
    }

    fn dfs(
        &self,
        t: isize,
        v: &str,
        m: usize,
        cache: &mut HashMap<(isize, String, usize), isize>,
    ) -> isize {
        if cache.contains_key(&(t, v.to_string(), m)) {
            return *cache.get(&(t, v.to_string(), m)).unwrap();
        }
        let m_val = self.distance_field[v]
            .keys()
            .into_iter()
            .filter(|i| m & (1 << self.nz_frate_v_index[*i]) == 0)
            .map(|i| {
                let new_time: isize = t - self.distance_field[v][i] - 1;
                if new_time <= 0 {
                    return 0;
                }
                return self.dfs(new_time, i, m | (1 << self.nz_frate_v_index[i]), cache)
                    + self.valves[self.v_index[i]].flow_rate * new_time;
            })
            .max()
            .unwrap_or(0);
        cache.insert((t, v.to_string(), m), m_val);
        return m_val;
    }

    fn calculate_max_released_pressure(&mut self) -> isize {
        let cache = &mut HashMap::new();
        return self.dfs(self.minutes, &self.start_valve, 0, cache);
    }

    fn calculate_max_released_pressure_with_elephant(&mut self) -> isize {
        let cache = &mut HashMap::new();
        let b = (1 << self.nz_frate_v_index.len()) - 1;
        return (0..((b + 1) / 2))
            .into_iter()
            .map(|i| {
                self.dfs(self.minutes, &self.start_valve, i, cache)
                    + self.dfs(self.minutes, &self.start_valve, b ^ i, cache)
            })
            .max()
            .unwrap_or(0);
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let valves = input.lines().map(Valve::from_str).collect::<Vec<Valve>>();
    let mut cave = Cave::new(30, valves, String::from("AA"));
    cave.calculate_lookup_tables().calculate_distance_field();

    // Part 1
    println!("Part 1: {:?}", cave.calculate_max_released_pressure());

    // Part 2
    cave.minutes = 26;
    println!(
        "Part 2: {:?}",
        cave.calculate_max_released_pressure_with_elephant()
    );
}
