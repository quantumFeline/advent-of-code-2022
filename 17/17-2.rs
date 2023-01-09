const N_ROCKS: usize = 1_000_000_000_001;
const FIGURES_ITER: usize = 100_000;
const SKIP_FIRST: usize = 2023;
const CHAMBER_W: usize = 7;

type Field = [Vec<char>; 7];
type Figure = Vec<String>;
#[derive(PartialEq, Clone, Copy, Debug)]
struct Coord {x: i32, y: i32}
#[derive(Clone, Copy, Debug)]
enum Shift {Left, Right, Down}
#[derive(Debug)]
struct FigurePosed<'a> {figure: &'a Figure, pos: Coord}

fn as_tuple(c: Coord) -> (i32, i32) {
    (c.x, c.y)
}

fn as_tuple_usize(c: Coord) -> (usize, usize) {
    (c.x as usize, c.y as usize)
}

fn max_h(field: &Field) -> usize {
    field.iter().map(|column| column.len()).max().unwrap()
}

fn coord_shift (coord: Coord, shift: Shift) -> Coord {
    match shift {
        Shift::Left => return Coord{x: coord.x-1, y: coord.y},
        Shift::Right => return Coord{x: coord.x+1, y: coord.y},
        Shift::Down => return Coord{x: coord.x, y: coord.y-1}
    }
}

fn is_free(coord: Coord, field: &Field) -> bool {
    let (x, y) = as_tuple(coord);
    if x < 0 || x >= CHAMBER_W as i32 || y <= 0 { // vertically infinite upwards; bottom layer is rocks
        return false;
    }
    let (xu, yu) = as_tuple_usize(coord);
    if y >= field[xu].len() as i32 {return true;}
    field[xu][yu] == '.'
}

fn to_absolute_c(figure: &FigurePosed, f_x: usize, f_y: usize) -> Coord {
    Coord{x: figure.pos.x + f_x as i32, y: figure.pos.y + f_y as i32}
}

fn move_figure(figure: &FigurePosed, shift: Shift, field: &Field) -> Coord {
    for (f_y, figure_line) in figure.figure.iter().enumerate() {
        for (f_x, figure_symbol) in figure_line.chars().enumerate() {
            if figure_symbol == '#' { // part of the figure we try to move, as opposed to a placeholder
                if !is_free(coord_shift(to_absolute_c(&figure, f_x, f_y), shift), field) {
                    //println!("Bam! {:?} can't move {:?} anymore (cell {} {} is {:?} in absolute coords)", figure, shift, f_x, f_y, to_absolute_c(&figure, f_x, f_y));
                    return figure.pos;
                }
            }
        }
    }
    return coord_shift(figure.pos, shift);
}

fn imprint_figure(figure: &FigurePosed, field: &mut Field) {
    for (f_y, figure_line) in figure.figure.iter().enumerate() {
        for (f_x, figure_symbol) in figure_line.chars().enumerate() {
            if figure_symbol == '#' { // part of the figure we try to move, as opposed to a placeholder
                let (x, y) = as_tuple_usize(to_absolute_c(&figure, f_x, f_y));
                // if any of the values are out of usize range, something went wrong, we shouldn't imprint such figure.
                while y >= field[x].len() {
                    field[x].push('.');// lengthen the vector as needed
                }
                field[x][y] = '#';
            }
        }
    }
}

fn print_field(field: &Field) {
    for y in (0..max_h(field)).rev() {
        print!("|");
        for x in 0..CHAMBER_W {
            if field[x].len() <= y {
                print!(".");
            } else {
                print!("{}", field[x][y]);
            }
        }
        println!("|");
    }
}

fn find_cycle(a: &[usize]) -> usize{
    for delta in 1_000..10_000 {
        let mut cycle = true;
        for i in SKIP_FIRST..SKIP_FIRST + delta {
            if a[i] != a[i+delta] {
                cycle = false;
                break;
            }
        }
        if cycle {
            return delta
        }
    }
    unreachable!()
}

fn sum(a: &[usize]) -> usize {
    a.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let rocks: Vec<Figure> = vec!["..####.\n.......\n.......\n.......",
                                  "...#...\n..###..\n...#...\n.......",
                                  "..###..\n....#..\n....#..\n.......",
                                  "..#....\n..#....\n..#....\n..#....",
                                  "..##...\n..##...\n.......\n......."].iter().map(|s| s.split("\n").map(|l| String::from(l)).collect::<Vec<_>>()).collect();

    let wind = include_str!("../input/17.txt").chars().collect::<Vec<char>>();
    let mut field: Field = Default::default();
    for i in 0..CHAMBER_W {field[i].push('-');}

    let mut delta_heights = [0; FIGURES_ITER+1];
    let mut wind_i = 0;
    let mut cur_figure_i = 0;
    let mut next = true;
    let mut rock = FigurePosed {figure: &rocks[0], pos: Coord{x: 0, y: 0}};

    while cur_figure_i <= FIGURES_ITER {
        let wind_step = *wind.get(wind_i).unwrap();

        if next {
            // check again if we still can't fall down after shift right or left
            let fall_pos = move_figure(&rock, Shift::Down, &field);
            if fall_pos != rock.pos {
                next = false;
                continue;
            }

            let prev_height = max_h(&field);
            imprint_figure(&rock, &mut field);
            let height = max_h(&field);
            delta_heights[cur_figure_i] = height - prev_height;
            rock = FigurePosed {figure: &rocks[cur_figure_i % rocks.len()], pos: Coord{x: 0, y: height as i32 + 3}};
            next = false;
            cur_figure_i += 1;

            if cur_figure_i % 10_000 == 0 {
                println!("Figure #{:?}; Height reached = {:?}", cur_figure_i, max_h(&field));
            }
        }

        if wind_step == '<' {
            rock.pos = move_figure(&rock, Shift::Left, &field);
        } else if wind_step == '>' {
            rock.pos = move_figure(&rock, Shift::Right, &field);
        } else {unreachable!()};

        let fall_pos = move_figure(&rock, Shift::Down, &field);
        if fall_pos == rock.pos { // figure stopped falling
            next = true;
        } else {
            rock.pos = fall_pos;
        }

        wind_i = (wind_i + 1) % wind.len();
    }
    println!("{:?}", max_h(&field)-1);
    let w = find_cycle(&delta_heights);
    println!("arr begin: {:?}",&delta_heights[..100]);
    println!("cycle of len {:?} starting {:?}: {:?}", w, SKIP_FIRST, &delta_heights[SKIP_FIRST..SKIP_FIRST+w]);
    let skip_sum = sum(&delta_heights[..SKIP_FIRST]);
    let cycle_sum = sum(&delta_heights[SKIP_FIRST..SKIP_FIRST+w]);
    let n_cycles = (N_ROCKS - SKIP_FIRST) / w;
    let tail_len = (N_ROCKS - SKIP_FIRST) % w;
    let tail_sum = sum(&delta_heights[SKIP_FIRST+w..SKIP_FIRST+w+tail_len]);
    println!("skip = {} (len={}) cycle = {} (len={}) n={} tail = {} (len={})", skip_sum, SKIP_FIRST, cycle_sum, w, n_cycles, tail_sum, tail_len);
    println!("Total sum: {}", skip_sum as u128 + cycle_sum as u128 * n_cycles as u128 + tail_sum as u128);
}