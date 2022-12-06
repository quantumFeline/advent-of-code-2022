use std::cmp::max;

fn main() {
    let input = include_str!("../input/6.txt");
    let mut last_pair: Option<(usize, usize)> = None;
    for (i, symbol) in input.chars().enumerate() {
        for (j, prev_symbol) in input.as_bytes()[max(i as i32-14,0) as usize..i].iter().rev().enumerate() {
            if *prev_symbol as char == symbol {
                if let Some(pair) = last_pair {
                    if pair.0 <= i-j-1 {
                        last_pair = Some((i-j-1, i));
                    }
                } else {
                    last_pair = Some((i-j-1, i));
                }
                break;
            }
        }

        match last_pair {
            Some(pair) => if i - pair.0 >= 14 {
                println!("{} {}", i+1, &input[i-14..i+1]); //+1 cause humans count from 1
                break;
            },
            None => if i >= 13  {
                println!("{} {}", i+1, &input[i-14..i+1]);
                break;
            }
        }
    }
}