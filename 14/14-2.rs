use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

fn to_tuple(x: &str) -> Coord {
    let mut xi = x.split(",").map(|i| i.parse::<usize>().unwrap());
    Coord{ x: xi.next().unwrap(), y: xi.next().unwrap()}
}

fn range_unsigned(x1: usize, x2:usize) -> std::ops::Range<usize> {
    if x1 < x2 {
        return x1..x2+1;
    } else {
        return x2..x1+1;
    }
}

fn main() {
    const WIDTH: usize = 700;
    let mut arr: [[bool; WIDTH]; 200] = [[false; WIDTH]; 200];
    let input = include_str!("../input/14.txt").lines().map(|line| line.split(" -> ").map(|x| to_tuple(x)).collect::<Vec<Coord>>());

    let mut floor = 0;
    // parsing
    for shelf in input {
        println!("{:?}", shelf);
        let mut line_begin = shelf.first().unwrap();
        floor = std::cmp::max(line_begin.y, floor);
        for line_end in &shelf[1..] {
            if line_begin.x != line_end.x {
                for x in range_unsigned(line_begin.x, line_end.x) {
                    //println!("{:?}", range_unsigned(line_begin.x, line_end.x));
                    arr[line_begin.y][x] = true;
                }
            } else {
                for y in range_unsigned(line_begin.y, line_end.y) {
                    //println!("{:?}", range_unsigned(line_begin.y, line_end.y));
                    arr[y][line_begin.x] = true;
                }
            }
            line_begin = line_end;
            floor = std::cmp::max(line_begin.y, floor);
        }
    }

    //floor
    floor += 2;
    for x in 0..WIDTH {
        arr[floor][x] = true;
    }

    //dfs
    let left = |c: Coord| -> Coord {Coord{x:c.x-1, y:c.y+1}};
    let down = |c: Coord| -> Coord {Coord{x:c.x, y:c.y+1}};
    let right = |c: Coord| -> Coord {Coord{x:c.x+1, y:c.y+1}};
    let free = |arr: &[[bool; WIDTH]], c: Coord| -> bool {!arr[c.y][c.x]};

    let mut grain: Coord = Coord{x: 500, y: 0};
    let mut fallen = 0;
    let mut path: VecDeque<Coord> = VecDeque::new();
    loop {
        path.push_back(grain);
        if free(&arr, down(grain)) {
            grain = down(grain);
        } else if free(&arr, left(grain)) {
            grain = left(grain);
        } else if free(&arr, right(grain)) {
            grain = right(grain);
        } else {
            fallen += 1;
            arr[grain.y][grain.x] = true;
            path.pop_back();
            if !path.is_empty() {grain = path.pop_back().unwrap();}
            else {break;}
        }
    }

    println!("{}", fallen);
}