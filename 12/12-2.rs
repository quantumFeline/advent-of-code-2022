use std::collections::VecDeque;

type Coord = (usize, usize);

fn main() {

    let input = include_str!("../input/12.txt");
    let hills: Vec<Vec<u32>> = input.split("\n").map(|line| line.chars().
    map(|c| if c == 'S' {0} else if c == 'E' {26} else {c as u32 - 'a' as u32}).collect::<Vec<u32>>()).collect();
    let row_len = hills[0].len() + 1; // including \n
    let starts : Vec<Coord> = input.chars().enumerate().filter(|(_i, c)| *c == 'S' || *c == 'a').map(|(i, _c)| (i % row_len, i / row_len)).collect();
    println!("{:?}", starts);
    let end = [input.chars().position(|c| c == 'E').unwrap()].map(|i| (i % row_len, i / row_len))[0];
    let mut distances = vec![vec![100_000; hills[0].len()]; hills.len()];
    println!("s={:?} e={:?}", starts, end);

    let w = hills[0].len();
    let h = hills.len();

    let get_neighbours = |x: Coord| -> Vec<Coord>{
        let mut vec_ans = vec![];
        if x.0 > 0   {vec_ans.push((x.0 - 1, x.1))};
        if x.0 < w-1 {vec_ans.push((x.0 + 1, x.1))};
        if x.1 > 0   {vec_ans.push((x.0, x.1 - 1))};
        if x.1 < h-1 {vec_ans.push((x.0, x.1 + 1))};
        vec_ans
    };

    let hill_height = |coord: Coord| -> u32 {hills[coord.1][coord.0]};
    let heigh_adj = |cur: Coord, next: Coord| -> bool {hill_height(cur) as i32 - hill_height(next) as i32 >= -1};
    let dist = |distances: &Vec<Vec<i32>>, coord: Coord| -> i32 {distances[coord.1][coord.0]};


    for start in starts.iter() {
        distances[start.1][start.0] = 0;
    }
    let mut q = VecDeque::from(starts);

    while !q.is_empty() {
        let cur_hill = *q.front().unwrap();
        let cur_dist = if hill_height(cur_hill) == 0 {0} else {dist(&distances, cur_hill)};
        for neighbour in get_neighbours(cur_hill) {
            if heigh_adj(cur_hill, neighbour) && dist(&distances, neighbour) == 100_000 {
                distances[neighbour.1][neighbour.0] = cur_dist as i32 + 1;
                q.push_back(neighbour);
            }
        }
        q.pop_front();
        println!("hill={:?} h={} d={}", cur_hill, hill_height(cur_hill), cur_dist);
    }
    //println!("{:?}", distances);
    println!("{}", distances[end.1][end.0]);
}