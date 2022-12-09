use std::collections::HashSet;

type Coord = (i32, i32);

fn to_tuple(x: &str) -> (char, i32) {
    let x_vec = x.split(" ").collect::<Vec<_>>();
    (x_vec[0].chars().nth(0).unwrap(), x_vec[1].parse::<i32>().unwrap())
}

fn move_grid(pos: Coord, dir: char) -> Coord{
    let mut new_pos = pos;
    match dir {
        'L' => new_pos.0 -= 1,
        'R' => new_pos.0 += 1,
        'U' => new_pos.1 += 1,
        'D' => new_pos.1 -= 1,
        _ => panic!()
    }
    new_pos
}

fn dist(pos1: Coord, pos2: Coord) -> i32{
    std::cmp::max((pos1.1 - pos2.1).abs(), (pos1.0 - pos2.0).abs())
}


fn main() {
    let input = include_str!("../input/9.txt").lines().map(|x| to_tuple(x)).collect::<Vec<_>>();
    println!("{:?}",input);

    let mut visited_cells: HashSet<Coord> = HashSet::new();

    let mut head = (0,0);
    let mut tail = (0,0);

    for (dir, steps) in input {
        for _i in 0..steps {
            let new_head = move_grid(head, dir);
            if dist(tail, new_head) > 1 {
                tail = head;
            }
            head = new_head;
            visited_cells.insert(tail);
        }
    }
    println!("{}", visited_cells.len());
}