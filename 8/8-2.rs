fn add_nonblocked(blocked: &mut bool, house_h: u32, cur_h: u32)-> u32 {
    let mut ans = 1;
    if *blocked {
        ans = 0;
    } else if house_h <= cur_h {
        *blocked = true; // but that tree still counts
    }
    ans
}

fn count_scenery(house_h: u32, forest: &Vec<u32>) -> u32 {
    let mut blocked = false;
    forest.iter().fold(0, |acc, h| acc + add_nonblocked(&mut blocked, house_h, *h))
}

fn main() {
    let forest: Vec<Vec<u32>> = include_str!("../input/8.txt").split("\n").map(|tree_line| tree_line.chars().
                                                map(|tree| tree.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
    let width = forest[0].len();
    let height = forest.len();
    let mut max_scenery = 0;

    for (y,tree_line) in forest.iter().enumerate() { // skip first and last
        for (x, house_h) in tree_line.iter().enumerate() {
            if x == 0 || y == 0 || x == width-1 || y == height-1 {
                continue; // all 0 anyway
            }
            let scenery_top = count_scenery(*house_h, &forest[..y].iter().map(|line| line[x]).rev().collect::<Vec<_>>());
            let scenery_bottom = count_scenery(*house_h, &forest[y+1..].iter().map(|line| line[x]).collect::<Vec<_>>());
            let scenery_left = count_scenery(*house_h, &tree_line[..x].iter().rev().map(|x| *x).collect::<Vec<_>>());
            let scenery_right = count_scenery(*house_h, &tree_line[x+1..].to_vec());
            let total_current_scenery = scenery_top * scenery_bottom * scenery_left * scenery_right;
            max_scenery = std::cmp::max(total_current_scenery, max_scenery);
            //println!("({},{}): {} = {} * {} * {} * {}", x, y, total_current_scenery, scenery_top, scenery_bottom, scenery_left, scenery_right);
        }
    }
    println!("{}", max_scenery);
}