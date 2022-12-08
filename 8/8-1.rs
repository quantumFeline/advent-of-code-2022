fn main() {
    let forest: Vec<Vec<u32>> = include_str!("../input/8.txt").split("\n").map(|tree_line| tree_line.chars().map(|tree| tree.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
    let mut hidden_trees = 0;
    let width = forest[0].len();
    let height = forest.len();

    for (y,tree_line) in forest.iter().enumerate() { // skip first and last
        for (x, tree_h) in tree_line.iter().enumerate() {
            if x == 0 || y == 0 || x == width-1 || y == height-1 {
                continue;
            }
            let cover_top = forest[..y].iter().map(|line| line[x]).max().unwrap();
            let cover_bottom = forest[y+1..].iter().map(|line| line[x]).max().unwrap();
            let cover_left = tree_line[..x].iter().max().unwrap();
            let cover_right = tree_line[x+1..].iter().max().unwrap();
            if tree_h <= std::cmp::min(&cover_top, &cover_bottom) && tree_h <= std::cmp::min(&cover_left, &cover_right) {
                hidden_trees += 1;
            }
        }
    }
    println!("{}", width*height - hidden_trees);
}