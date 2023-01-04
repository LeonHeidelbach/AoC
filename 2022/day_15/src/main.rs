use std::fs;

enum MarkerTypes {
    BEACON,
    SENSOR,
}

struct Marker {
    x: isize,
    y: isize,
    d: isize,
}

impl Marker {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y, d: -1 }
    }

    fn from_str(input: &str) -> Self {
        let mut split = input.split(":");
        let mut s = Marker::parse(split.next().unwrap(), MarkerTypes::SENSOR);
        s.d = s.manhatten_distance(&Marker::parse(split.next().unwrap(), MarkerTypes::BEACON));
        return s;
    }

    fn parse(input: &str, m_type: MarkerTypes) -> Self {
        let (x, y) = match m_type {
            MarkerTypes::SENSOR => Marker::parse_coords(input, "Sensor at "),
            MarkerTypes::BEACON => Marker::parse_coords(input, " closest beacon is at "),
        };
        return Self::new(x, y);
    }

    fn parse_coords(input: &str, start_match: &str) -> (isize, isize) {
        let mut coords = input.trim_start_matches(start_match).split(", ");
        return (
            coords
                .next()
                .unwrap()
                .trim_start_matches("x=")
                .parse::<isize>()
                .unwrap(),
            coords
                .next()
                .unwrap()
                .trim_start_matches("y=")
                .parse::<isize>()
                .unwrap(),
        );
    }

    fn manhatten_distance(&self, other: &Marker) -> isize {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }

    fn intersection_points(&self, row: isize) -> Option<(isize, isize)> {
        let t = self.d - (self.y - row).abs();
        if t > 0 {
            return Some((self.x - t, self.x + t));
        }
        return None;
    }
}

fn calculate_ranges(m: &Vec<Marker>, row: isize) -> Vec<(isize, isize)> {
    let mut r: Vec<(isize, isize)> = vec![];
    for i in 0..m.len() {
        let s = m[i].intersection_points(row);
        if s.is_some() {
            r.push(s.unwrap());
        }
    }
    merge_range_intersections(&mut r);
    return r;
}

fn merge_range_intersections(r: &mut Vec<(isize, isize)>) {
    r.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    if r.len() < 2 {
        return;
    }
    let mut i = 0;
    while i < r.len() - 1 {
        if ranges_intersect(r[i], r[i + 1]) {
            r[i] = (r[i].0.min(r[i + 1].0), r[i + 1].1.max(r[i].1));
            r.remove(i + 1);
            i = 0;
            continue;
        }
        i += 1;
    }
}

fn ranges_intersect(r1: (isize, isize), r2: (isize, isize)) -> bool {
    return !(r1.0 > r2.0 && r1.0 > r2.1) && !(r1.1 < r2.0 && r1.1 < r2.1);
}

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let markers = input.lines().map(Marker::from_str).collect::<Vec<Marker>>();

    // Part 1
    let y = 2000000;
    let pos_num = calculate_ranges(&markers, y)
        .iter()
        .fold(0, |acc, (a, b)| acc + a.abs() + b.abs());
    println!("Part 1: {:?}", pos_num);

    // Part 2
    let freq_multiplier = 4000000;
    let u_bound = freq_multiplier;

    (0..u_bound)
        .map(|i| (i, calculate_ranges(&markers, i)))
        .filter(|(_, r)| r.len() > 1 && r[1].0 - r[0].1 == 2)
        .map(|(i, r)| (r[0].1 + 1, i))
        .for_each(|(x, y)| println!("Part 2: {:?}", x * u_bound + y));
}
