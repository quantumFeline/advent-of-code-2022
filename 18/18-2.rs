use std::convert::TryInto;
use std::collections::VecDeque;

type Grid3D = Vec<Vec<Vec<bool>>>;
type Coord = (usize, usize, usize);
const CUBE_SIDE: usize = 24;

fn get_cell_neighbours(c: Coord) -> Vec<Coord> {
    let (x,y,z) = c;
    let mut neighbours: Vec<Coord> = vec![];
    if z != 0           { neighbours.push((x  , y  , z-1))};
    if z != CUBE_SIDE-1 { neighbours.push((x  , y  , z+1))};
    if y != 0           { neighbours.push((x  , y-1, z  ))};
    if y != CUBE_SIDE-1 { neighbours.push((x  , y+1, z  ))};
    if x != 0           { neighbours.push((x-1, y,   z  ))};
    if x != CUBE_SIDE-1 { neighbours.push((x+1, y,   z  ))};
    neighbours
}

fn count_cell_air_neighbours(grid: &Grid3D, x: usize, y: usize, z: usize) -> usize {
    let up: usize       = if z == 0           { 0 } else { (!grid[x][y][z-1]).try_into().unwrap()};
    let down: usize     = if z == CUBE_SIDE-1 { 0 } else { (!grid[x][y][z+1]).try_into().unwrap()};
    let backward: usize = if y == 0           { 0 } else { (!grid[x][y-1][z]).try_into().unwrap()};
    let forward: usize  = if y == CUBE_SIDE-1 { 0 } else { (!grid[x][y+1][z]).try_into().unwrap()};
    let left: usize     = if x == 0           { 0 } else { (!grid[x-1][y][z]).try_into().unwrap()};
    let right: usize    = if x == CUBE_SIDE-1 { 0 } else { (!grid[x+1][y][z]).try_into().unwrap()};
    up + down + backward + forward + left + right
}

//debug purposes
fn print3D(grid: &Grid3D) {
    for side in grid {
        for line in side {
            println!("{}", line.iter().map(|x| if *x {"#"} else {"."}).collect::<Vec<_>>().join(""));
        }
        println!("");
    }
}

fn grid_air_sum(grid: &Grid3D, visited: &mut Grid3D) -> usize {
    let mut q: VecDeque<Coord> = VecDeque::new();
    q.push_back((0,0,0));
    let mut sides: usize = 0;
    while !q.is_empty() {
        let (x,y,z) = q.pop_front().unwrap();
        visited[x][y][z] = true;
        sides += 6 - count_cell_air_neighbours(&grid, x, y, z);
        for neighbour in get_cell_neighbours((x,y,z)).iter().filter(|(nx, ny, nz)| !grid[*nx][*ny][*nz]) { // don't go into lava
            let (nx, ny, nz) = *neighbour;
            if !visited[nx][ny][nz] {
                visited[nx][ny][nz] = true;
                q.push_back(*neighbour);
            }
        }
    }
    sides - CUBE_SIDE * CUBE_SIDE * 6
}

fn main() {
    let input = include_str!("../input/18.txt").lines().map(|line| line.split(",").map(|x| x.parse::<usize>().unwrap()+1).collect::<Vec<_>>());
    let mut grid3D: Grid3D = vec![vec![vec![false; CUBE_SIDE]; CUBE_SIDE]; CUBE_SIDE];
    let mut visited: Grid3D = vec![vec![vec![false; CUBE_SIDE]; CUBE_SIDE]; CUBE_SIDE];
    for cube in input {
        grid3D[cube[0]][cube[1]][cube[2]] = true;
    }
    println!("{}", grid_air_sum(&grid3D, &mut visited));
}