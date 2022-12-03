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
    for line in include_str!("./input/3.txt").lines() {
        let l = line.len();
        let c1 = HashSet::<char>::from_iter(line[0..l/2].chars());
        let c2 = HashSet::<char>::from_iter(line[l/2..l].chars());
        sum += val(c1.intersection(&c2).collect::<Vec<&char>>()[0]);
    }

    println!("{:?}",sum);
}