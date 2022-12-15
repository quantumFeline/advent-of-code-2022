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

    //println!("{}", n_cover);


//    // parsing
//    for shelf in input {
//        println!("{:?}", shelf);
//        let mut line_begin = shelf.first().unwrap();
//        for line_end in &shelf[1..] {
//            if line_begin.x != line_end.x {
//                for x in range_unsigned(line_begin.x, line_end.x) {
//                    //println!("{:?}", range_unsigned(line_begin.x, line_end.x));
//                    arr[line_begin.y][x] = true;
//                }
//            } else {
//                for y in range_unsigned(line_begin.y, line_end.y) {
//                    //println!("{:?}", range_unsigned(line_begin.y, line_end.y));
//                    arr[y][line_begin.x] = true;
//                }
//            }
//            line_begin = line_end;
//        }
//    }
//
//    //dfs
//    let left = |c: Coord| -> Coord {Coord{x:c.x-1, y:c.y+1}};
//    let down = |c: Coord| -> Coord {Coord{x:c.x, y:c.y+1}};
//    let right = |c: Coord| -> Coord {Coord{x:c.x+1, y:c.y+1}};
//    let free = |arr: &[[bool;600]], c: Coord| -> bool {!arr[c.y][c.x]};
//
//    let mut grain: Coord = Coord{x:500, y:0};
//    let mut fallen = 0;
//    let mut path: VecDeque<Coord> = VecDeque::new();
//    while grain.y < 199 { // fell down all the way to the bottom
//        path.push_back(grain);
//        if free(&arr, down(grain)) {
//            //println!("down from {} {}", grain.x, grain.y);
//            grain = down(grain);
//        } else if free(&arr, left(grain)) {
//            //println!("left from {} {}", grain.x, grain.y);
//            grain = left(grain);
//        } else if free(&arr, right(grain)) {
//            //println!("right from {} {}", grain.x, grain.y);
//            grain = right(grain);
//        } else {
//            fallen += 1;
//            arr[grain.y][grain.x] = true;
//            //println!("fell at {} {}", grain.x, grain.y);
//            path.pop_back();
//            grain = path.pop_back().unwrap();
//        }
//    }
//
//    println!("{}", fallen);
//    for y in arr {
//        for x in y {
//            print!{"{}", if x {"#"} else {"."}};
//        }
//        println!("");
//    }
}
