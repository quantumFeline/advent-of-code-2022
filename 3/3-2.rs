use std::collections::HashSet;
use std::iter::FromIterator;

fn val(item: &char) -> u32 {
    let mut index = *item as u32;
    if index >= 'A' as u32 && index <= 'Z' as u32 {
        index -= 'A' as u32;
        index += 27;
    } else {
        index -= 'a' as u32;
        index += 1;
    }
    index
}

fn main() {

    let mut sum = 0;
    let lines = include_str!("./input/3.txt").lines().collect::<Vec<&str>>();
    for line_i in (0..lines.len()).step_by(3) {
        let c1 = HashSet::<char>::from_iter(lines[line_i].chars());
        let c2 = HashSet::<char>::from_iter(lines[line_i+1].chars());
        let i1: HashSet::<char> = c1.intersection(&c2).map(|item| *item).collect();
        let c3 = HashSet::<char>::from_iter(lines[line_i+2].chars());
        sum += val(i1.intersection(&c3).collect::<Vec<&char>>()[0]);
        //println!("{:?}",sum);
    }

    println!("{:?}",sum);
}