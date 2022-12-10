fn signal_change(i: i32, x: i32, next_signal_point: &mut i32) -> i32 {
    let mut signal_str_diff = 0;
    if i == *next_signal_point {
        signal_str_diff = x * i;
        *next_signal_point += 40;
        println!("i={} x={} str={} next={}", i, x, signal_str_diff, *next_signal_point);
    }
    signal_str_diff
}

fn main() {
    let input = include_str!("../input/10.txt").lines();

    let mut next_signal_point = 20;
    let mut i = 0;
    let mut x = 1;
    let mut signal_str = 0;
    for line in input {
        match line[..4].as_ref() {
            "noop" =>{
                i += 1;
                signal_str += signal_change(i, x, &mut next_signal_point);
            },
            "addx" => {
                i += 1;
                signal_str += signal_change(i, x, &mut next_signal_point);
                i += 1;
                signal_str += signal_change(i, x, &mut next_signal_point);
                x += line[5..].parse::<i32>().unwrap();
                //println!("dx={} x={}", line[5..].parse::<i32>().unwrap(), x);
            },
            _ => panic!()
        }
        if next_signal_point > 220 {
            break;
        }
    }
    println!("{}", signal_str);
}