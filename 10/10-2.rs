fn signal_change(i: &mut i32, x: i32, next_signal_point: &mut i32) -> i32 {
    if *i % 40 == 0 { print!("\n")};
    print!("{}", if (x - *i % 40).abs() <= 1 {'#'} else {'.'});

    *i += 1;
    let mut signal_str_diff = 0;
    if *i == *next_signal_point {
        signal_str_diff = x * *i;
        *next_signal_point += 40;
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
                signal_str += signal_change(&mut i, x, &mut next_signal_point);
            },
            "addx" => {
                signal_str += signal_change(&mut i, x, &mut next_signal_point);
                signal_str += signal_change(&mut i, x, &mut next_signal_point);
                x += line[5..].parse::<i32>().unwrap();
            },
            _ => panic!()
        }
    }
    println!("\n{}", signal_str);
}