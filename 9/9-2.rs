use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.x, self.y)
    }
}

fn move_grid(pos: Coord, dir: char) -> Coord {
    let mut new_pos = pos;
    match dir {
        'L' => new_pos.x -= 1,
        'R' => new_pos.x += 1,
        'U' => new_pos.y += 1,
        'D' => new_pos.y -= 1,
        _ => panic!()
    }
    new_pos
}

fn move_vim(pos: Coord, dir: char) -> Coord {
    let mut new_pos = pos;
    match dir {
        'H' => new_pos.x -= 1,
        'L' => new_pos.x += 1,
        'K' => new_pos.y += 1,
        'J' => new_pos.y -= 1,
        'Y' => {new_pos.x -=1; new_pos.y += 1},
        'U' => {new_pos.x +=1; new_pos.y += 1},
        'B' => {new_pos.x -=1; new_pos.y -= 1},
        'N' => {new_pos.x +=1; new_pos.y -= 1},
        '.' => {},
        _ => panic!()
    }
    new_pos
}

struct Rope {
    segs: [Coord; 10],
    visited_cells: HashSet<Coord>,
    vim_snap: Vec<Vec<char>>
}

impl Rope {
    const LEN: usize = 10;

    fn new() -> Self {
        Rope{segs: [Coord{x:0,y:0}; Rope::LEN], visited_cells: HashSet::new(),
            vim_snap:
"NNJBB
N...B
L...H
U...Y
UUKYY".split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
        }
    }

    fn head(&self) -> Coord {
        self.segs[0]
    }

    fn tail(&self) -> Coord {
        *self.segs.last().unwrap()
    }

    fn move_grid(&mut self, dir: char) {
        self.segs[0] = move_grid(self.head(), dir);
        for i in 1..Rope::LEN {
            let diff_x: i32 = self.segs[i].x - self.segs[i-1].x;
            let diff_y: i32 = - self.segs[i].y + self.segs[i-1].y; // up is higher numbers, but ot in the vim_snap arr!
            self.segs[i] = move_vim(self.segs[i], self.vim_snap[(2+diff_y) as usize][(2+diff_x) as usize]);
        }
        self.visited_cells.insert(self.tail());
    }

    fn total_visited(self) -> usize {
        self.visited_cells.len()
    }
}

fn to_tuple(x: &str) -> (char, i32) {
    let x_vec = x.split(" ").collect::<Vec<_>>();
    (x_vec[0].chars().nth(0).unwrap(), x_vec[1].parse::<i32>().unwrap())
}


fn main() {
    let input = include_str!("../input/9.txt").lines().map(|x| to_tuple(x)).collect::<Vec<_>>();
    let mut rope = Rope::new();

    for (dir, steps) in input {
        for _i in 0..steps {
            rope.move_grid(dir);
        }
    }
    println!("{}", rope.total_visited());
}