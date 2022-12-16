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
                n_cover += 1;
                break;
            }
        }
    }

    println!("{}", n_cover);
}
