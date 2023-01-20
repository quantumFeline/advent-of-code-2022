static CUBE_SIDE: usize = 4;
#[derive(PartialEq, Clone, Copy, Debug)]
struct Coord {x: usize, y: usize}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {Left, Right, Up, Down}

#[derive(Clone, Copy, Debug)]
enum Side {Left, Right, Top, Bottom, Front, Back}

fn to_value(d: Direction) -> usize {
    match d {
        Direction::Right => 0,
        Direction::Down  => 1,
        Direction::Left  => 2,
        Direction::Up    => 3
    }
}

fn switch_side(s: Side, d: Direction) -> (Side, Direction) {
    match s {
        Side::Left   => {[(Side::Front, Direction::Right), (Side::Bottom,  Direction::Right), (Side::Back,  Direction::Right), (Side::Top,    Direction::Right)] [to_value(d)]},
        Side::Right  => {[(Side::Back,  Direction::Right), (Side::Bottom,  Direction::Left),  (Side::Front, Direction::Left),  (Side::Top,    Direction::Left)]  [to_value(d)]},
        Side::Top    => {[(Side::Right, Direction::Down),  (Side::Front,   Direction::Down),  (Side::Left,  Direction::Down),  (Side::Back,   Direction::Up)]    [to_value(d)]},
        Side::Bottom => {[(Side::Right, Direction::Up),    (Side::Back,    Direction::Down),  (Side::Left,  Direction::Up),    (Side::Front,  Direction::Up)]    [to_value(d)]},
        Side::Front  => {[(Side::Right, Direction::Right), (Side::Bottom,  Direction::Down),  (Side::Left,  Direction::Left),  (Side::Top,    Direction::Up)]    [to_value(d)]},
        Side::Back   => {[(Side::Right, Direction::Left),  (Side::Top,     Direction::Down),  (Side::Left,  Direction::Right), (Side::Bottom, Direction::Up)]    [to_value(d)]},
    }
}

fn is_crossing(pc: PC, w: usize, h: usize) -> bool {
    (pc.0.x == 0   && pc.2 == Direction::Left)  ||
    (pc.0.x == w-1 && pc.2 == Direction::Right) ||
    (pc.0.y == 0   && pc.2 == Direction::Up)    ||
    (pc.0.y == h-1 && pc.2 == Direction::Down)
}


#[derive(Clone, Copy, Debug)]
struct PC (Coord, Side, Direction);

type Dungeon = Vec<Vec<char>>;
type DungeonCube = [Vec<Vec<char>>;6];

fn pad(s: &str, n: usize) -> String {
    assert!(s.len() <= n);
    format!("{}{}", s, " ".repeat(n - s.len()))
}

fn turn (d: Direction, turn: char) -> Direction {
    match turn {
        'R' => {
            match d {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left
            }
        },
        'L' => {
            match d {
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left
            }
        },
        _ => unreachable!()
    }
}

fn step_cube(pc: PC, w: usize, h: usize) -> PC {
    let (coord, side, dir) = (pc.0, pc.1, pc.2);
    if is_crossing(pc) {
        let (new_side, new_dir) = switch_side(side, dir);
    }
}

fn next_forward(dungeon: &Dungeon, pc: PC) -> PC {
    let mut next_pc = pc;
    loop {
        next_pc = step_cube(next_pc, dungeon[0].len(), dungeon.len());
        //println!("checking d[{}][{}] == \"{}\"", next_pc.0.y, next_pc.0.x, dungeon[next_pc.0.y][next_pc.0.x]);
        match dungeon[next_pc.0.y][next_pc.0.x] {
            '.' => return next_pc,
            '#' => return pc,
            ' ' => {},
            _ => unreachable!()
        }
    }
}

fn main() {

    let full_input = include_str!("../input/22.txt");
    let split_point = full_input.find(char::is_numeric).unwrap();
    let dungeon_lines = &full_input[..split_point].trim_matches('\n').split("\n").collect::<Vec<_>>();
    let dungeon: Dungeon = dungeon_lines.iter().map(|line| pad(line, dungeon_lines.iter().map(|l| l.len()).max().unwrap()).chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut dungeon_cube: DungeonCube = vec![];
    for top in (0..dungeon.len()).step_by(CUBE_SIDE) {
        for left in (0..dungeon[0].len()).step_by(CUBE_SIDE) {
            if dungeon[top][left] != ' ' { // a valid cube side found
                let side = (0..CUBE_SIDE).map(|step_y| dungeon[top + step_y][left..left+CUBE_SIDE].iter().collect::<Vec<_>>()).collect::<Vec<_>>();
                dungeon_cube.push(side);
            }
        }
    }
    for side in dungeon_cube.iter() {
        println!("{:?}\n\n", side);
    }

//    let path = &full_input[split_point..];
//    let mut pos = PC(Coord {x: dungeon[0].iter().position(|&c| c == '.').unwrap(), y: 0}, Direction::Right);
//    println!("{:?} \n{:?}", dungeon, path);
//    println!("{:?}", pos);
//
//    let step_multiple = |n: &String, pc: &mut PC| {
//        if n == "" {return;}
//        let n_int = n.parse::<usize>().unwrap();
//        println!("stepping {} steps:", n_int);
//        for _ in 0..n_int {
//            *pc = next_forward(&dungeon, *pc);
//        }
//    };
//
//    let mut number = String::from("");
//    for symbol in path.chars() {
//        println!("cur symbol: {}  cur pos: {:?}", symbol, pos);
//        match symbol {
//            'R' | 'L' => {
//                step_multiple(&number, &mut pos);
//                pos.1 = turn(pos.1, symbol);
//                number = String::from("");
//            },
//            '0'..='9' => {
//                number = format!("{}{}", number, symbol.to_string());
//            },
//            _ => unreachable!()
//        }
//        println!("number: {} updated pos: {:?}", number, pos);
//    }
//    step_multiple(&number, &mut pos);
//    println!("{:?}", pos);
//    println!("Answer: {}", 1000 * (pos.0.y + 1) + 4 * (pos.0.x + 1) + to_value(pos.1));
}