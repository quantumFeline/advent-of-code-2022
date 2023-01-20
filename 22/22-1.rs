#[derive(PartialEq, Clone, Copy, Debug)]
struct Coord {x: usize, y: usize}

#[derive(Clone, Copy, Debug)]
enum Direction {Left, Right, Up, Down}

fn to_value(d: Direction) -> usize {
    match d {
        Direction::Right => 0,
        Direction::Down  => 1,
        Direction::Left  => 2,
        Direction::Up    => 3
    }
}

#[derive(Clone, Copy, Debug)]
struct PC (Coord, Direction);

type Dungeon = Vec<Vec<char>>;

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

fn step_torus(pc: PC, w: usize, h: usize) -> PC {
    let (coord, dir) = (pc.0, pc.1);
    PC(match dir {
        Direction::Left  => {if coord.x == 0   {Coord{x:w-1,     y: coord.y}} else {Coord{x: coord.x-1, y: coord.y}}},
        Direction::Right => {if coord.x == w-1 {Coord{x:0,       y: coord.y}} else {Coord{x: coord.x+1, y: coord.y}}},
        Direction::Up    => {if coord.y == 0   {Coord{x:coord.x, y: h-1}}     else {Coord{x: coord.x,   y: coord.y-1}}},
        Direction::Down  => {if coord.y == h-1 {Coord{x:coord.x, y: 0}}       else {Coord{x: coord.x,   y: coord.y+1}}}
    }, dir)
}

fn next_forward(dungeon: &Dungeon, pc: PC) -> PC {
    let mut next_pc = pc;
    loop {
        next_pc = step_torus(next_pc, dungeon[0].len(), dungeon.len());
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
    let path = &full_input[split_point..];
    let mut pos = PC(Coord {x: dungeon[0].iter().position(|&c| c == '.').unwrap(), y: 0}, Direction::Right);
    println!("{:?} \n{:?}", dungeon, path);
    println!("{:?}", pos);

    let step_multiple = |n: &String, pc: &mut PC| {
        if n == "" {return;}
        let n_int = n.parse::<usize>().unwrap();
        println!("stepping {} steps:", n_int);
        for _ in 0..n_int {
            *pc = next_forward(&dungeon, *pc);
        }
    };

    let mut number = String::from("");
    for symbol in path.chars() {
        println!("cur symbol: {}  cur pos: {:?}", symbol, pos);
        match symbol {
            'R' | 'L' => {
                step_multiple(&number, &mut pos);
                pos.1 = turn(pos.1, symbol);
                number = String::from("");
            },
            '0'..='9' => {
                number = format!("{}{}", number, symbol.to_string());
            },
            _ => unreachable!()
        }
        println!("number: {} updated pos: {:?}", number, pos);
    }
    step_multiple(&number, &mut pos);
    println!("{:?}", pos);
    println!("Answer: {}", 1000 * (pos.0.y + 1) + 4 * (pos.0.x + 1) + to_value(pos.1));
}