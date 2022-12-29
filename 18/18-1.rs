use std::convert::TryInto;
type Grid3D = Vec<Vec<Vec<bool>>>;
const CUBE_SIDE: usize = 25;

fn count_cell_lava_neighbours(grid: &Grid3D, x: usize, y: usize, z: usize) -> usize {
    let up: usize       = if z == 0           { 0 } else { (grid[x][y][z-1]).try_into().unwrap()};
    let down: usize     = if z == CUBE_SIDE-1 { 0 } else { (grid[x][y][z+1]).try_into().unwrap()};
    let backward: usize = if y == 0           { 0 } else { (grid[x][y-1][z]).try_into().unwrap()};
    let forward: usize  = if y == CUBE_SIDE-1 { 0 } else { (grid[x][y+1][z]).try_into().unwrap()};
    let left: usize     = if x == 0           { 0 } else { (grid[x-1][y][z]).try_into().unwrap()};
    let right: usize    = if x == CUBE_SIDE-1 { 0 } else { (grid[x+1][y][z]).try_into().unwrap()};
    up + down + backward + forward + left + right
}

fn grid_sum(grid: &Grid3D) -> usize {
    let mut sides: usize = 0;
    for (x, side) in grid.iter().enumerate() {
        for (y, line) in side.iter().enumerate() {
            for (z, cell) in line.iter().enumerate() {
                if *cell {
                    let neighbours = count_cell_lava_neighbours(&grid, x, y, z);
                    sides += 6 - neighbours;
                }
            }
        }
    }
    sides
}

fn main() {
    let input = include_str!("../input/18.txt").lines().map(|line| line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>());
    let mut grid3D: Grid3D = vec![vec![vec![false; CUBE_SIDE]; CUBE_SIDE]; CUBE_SIDE];
    for cube in input {
        grid3D[cube[0]][cube[1]][cube[2]] = true;
    }
    println!("{}", grid_sum(&grid3D));
}