const SEARCH_RANGE_MIN: i32 = 0;
const SEARCH_RANGE_MAX: i32 = 4_000_000;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

type Segment = (i32, i32);

fn to_tuple(x: &str) -> Coord {
    let mut xi = x.split(", ").map(|i| i[2..].parse::<i32>().unwrap());
    Coord{ x: xi.next().unwrap(), y: xi.next().unwrap()}
}

fn distance(a: Coord, b: Coord) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

fn intersection(sensor: Coord, sensor_range: u32, row: i32) -> Option<Segment> {
    let dist_to_row = (sensor.y - row).abs();
    let half_span = sensor_range as i32 - dist_to_row;
    if half_span < 0 {None}
    else {Some((std::cmp::max(SEARCH_RANGE_MIN, sensor.x - half_span), std::cmp::min(SEARCH_RANGE_MAX, sensor.x + half_span)+1))}
}

fn merge_segments(segments: &mut Vec<Segment>) -> Vec<Segment> {
    segments.sort_by_key(|s| s.0);
    let mut ans = vec![];
    let mut cur = (-1,-1);
    for s in segments {
        if s.0 > cur.1 {
            if cur.0 < cur.1 {ans.push(cur)};
            cur = *s;
        } else if s.1 > cur.1 {
            cur.1 = s.1;
        }
    }
    if cur.0 < cur.1 {ans.push(cur)};
    ans
}

fn main() {
    let s1 = "Sensor at ";
    let s2 = ": closest beacon is at ";
    let input = include_str!("../input/15.txt").lines().map(|line| [line[s1.len()..].split(s2).map(|x| to_tuple(x))].
                                                            map(|mut coords| (coords.next().unwrap(), coords.next().unwrap()))[0]).collect::<Vec<_>>();

    for y in SEARCH_RANGE_MIN..SEARCH_RANGE_MAX {
        let row_lit = merge_segments(&mut input.iter().map(|(sensor,beacon)| intersection(*sensor, distance(*sensor, *beacon), y as i32)).filter_map(|s| s).collect::<Vec<_>>());
        if row_lit.len() != 1 {
            println!("Found! {} {}", row_lit[0].1, y);
            println!("f={}", row_lit[0].1 as i128 * 4_000_000 as i128 + y as i128);
        }
    }
}
