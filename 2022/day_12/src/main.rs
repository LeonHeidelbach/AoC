use std::{cmp, fs, str};

struct Map {
    map: Vec<Vec<isize>>,
    marker_map: Vec<Vec<isize>>,
    shortest_path_from_start: isize,
    overall_shortest_path: isize,
    start_pos: Vec<(isize, isize)>,
    end_pos: (isize, isize),
}

impl Map {
    fn new() -> Self {
        Self {
            map: vec![],
            marker_map: vec![],
            shortest_path_from_start: 0,
            overall_shortest_path: 0,
            start_pos: vec![],
            end_pos: (0, 0),
        }
    }

    fn parse_input(&mut self, input_lines: &mut str::Lines) {
        for (i, line) in input_lines.enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                match c {
                    'a' => self.start_pos.push((i as isize, j as isize)),
                    'S' => {
                        self.start_pos.insert(0, (i as isize, j as isize));
                        row.push('a' as isize);
                        continue;
                    }
                    'E' => {
                        self.end_pos = (i as isize, j as isize);
                        row.push('z' as isize);
                        continue;
                    }
                    _ => (),
                }
                row.push(c as isize)
            }
            self.map.push(row);
        }
        self.reset_marker_map();
    }

    fn get_map_val(map: &Vec<Vec<isize>>, pos: (isize, isize)) -> isize {
        return map[pos.0 as usize][pos.1 as usize];
    }

    fn set_map_val(map: &mut Vec<Vec<isize>>, pos: (isize, isize), val: isize) {
        map[pos.0 as usize][pos.1 as usize] = val;
    }

    fn reset_marker_map(&mut self) {
        self.marker_map = vec![vec![-1; self.map[0].len()]; self.map.len()];
    }

    fn is_eligible_field(&self, p_old: (isize, isize), p_new: (isize, isize)) -> bool {
        let p_old_val = Map::get_map_val(&self.map, p_old);
        let p_new_val = Map::get_map_val(&self.map, p_new);
        let m_map_val = Map::get_map_val(&self.marker_map, p_new);
        return m_map_val == -1 && p_new_val - p_old_val <= 1;
    }

    fn evaluate_position(
        &mut self,
        bfs: &mut Vec<(isize, isize)>,
        pos: (isize, isize),
        new_pos: (isize, isize),
    ) -> bool {
        if new_pos.0 >= 0
            && new_pos.0 < self.map.len() as isize
            && new_pos.1 >= 0
            && new_pos.1 < self.map[0].len() as isize
            && self.is_eligible_field(pos, new_pos)
        {
            let pos_val = Map::get_map_val(&self.marker_map, pos);
            if new_pos == self.end_pos {
                self.shortest_path_from_start = pos_val + 1;
                self.overall_shortest_path = if self.overall_shortest_path == 0 {
                    self.shortest_path_from_start
                } else {
                    cmp::min(self.overall_shortest_path, self.shortest_path_from_start)
                };
                return true;
            }
            Map::set_map_val(&mut self.marker_map, new_pos, pos_val + 1);
            bfs.push(new_pos);
        }
        return false;
    }

    fn find_shortest_path(&mut self, start_pos: (isize, isize)) {
        let mut bfs: Vec<(isize, isize)> = vec![start_pos];
        Map::set_map_val(&mut self.marker_map, start_pos, 0);

        while bfs.len() > 0 {
            let mut new_bfs: Vec<(isize, isize)> = vec![];
            for pos in bfs {
                if self.evaluate_position(&mut new_bfs, pos, (pos.0 - 1, pos.1))
                    || self.evaluate_position(&mut new_bfs, pos, (pos.0 + 1, pos.1))
                    || self.evaluate_position(&mut new_bfs, pos, (pos.0, pos.1 - 1))
                    || self.evaluate_position(&mut new_bfs, pos, (pos.0, pos.1 + 1))
                {
                    return;
                }
            }
            bfs = new_bfs;
        }
    }

    fn find_all_shortest_paths(&mut self) {
        for i in (0..self.start_pos.len()).rev() {
            self.reset_marker_map();
            self.find_shortest_path(self.start_pos[i]);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let mut input_lines: str::Lines = input.lines();
    let mut map: Map = Map::new();

    map.parse_input(&mut input_lines);
    map.find_all_shortest_paths();

    // Part 1
    println!("Part 1: {:?}", map.shortest_path_from_start);

    // Part 2
    println!("Part 2: {:?}", map.overall_shortest_path);
}
