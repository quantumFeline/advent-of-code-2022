#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

fn to_tuple(x: &str) -> Coord {
    let mut xi = x.split(", ").map(|i| i[2..].parse::<i32>().unwrap());
    Coord{ x: xi.next().unwrap(), y: xi.next().unwrap()}
}

fn distance(a: Coord, b: Coord) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

fn main() {
    const ROW: i32 = 2_000_000;
    let s1 = "Sensor at ";
    let s2 = ": closest beacon is at ";
    let input = include_str!("../input/15.txt").lines().map(|line| [line[s1.len()..].split(s2).map(|x| to_tuple(x))].
                                                            map(|mut coords| (coords.next().unwrap(), coords.next().unwrap()))[0]).collect::<Vec<_>>();
    let mut n_cover = 0;
    for i in -1_000_000..6_000_000 {
        let cur_coord = Coord{x: i, y: ROW};
        for (sensor, beacon) in input.iter() {
            if *beacon == cur_coord {break;}
            if distance(*sensor, cur_coord) <= distance(*sensor, *beacon) {
                //println!("{:?}", cur_coord);
                n_cover += 1;
                break;
            }
            //println!("{:?}", distance(sensor, beacon));
        }
    }

    println!("{}", n_cover);


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
