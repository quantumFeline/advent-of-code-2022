use std::convert::TryInto;

fn mathmodulo(x: i64, d: usize) -> usize {
    let di = d as i64;
    (((x % di) + di) % di).try_into().unwrap()
}

fn main() {
    let input = include_str!("../input/20.txt").lines().enumerate().map(|(i, x)| (x.parse::<i64>().unwrap() * 811589153, i));
    let mut arr = input.clone().collect::<Vec<_>>();
    let len = arr.len();
    println!("{:?}", arr);
    for _mix_i in 0..10 {
        for elem in input.clone() {
            let i = arr.iter().position(|x| *x == elem).unwrap();
            let n = elem.0;
            arr.remove(i);
            let new_pos = mathmodulo(i as i64 + n, len-1);
            arr.insert(new_pos, elem);
        }
        println!("{:?}", arr);
    }
    let count_from = arr.iter().position(|x| x.0 == 0).unwrap() as i64;
    let fin_pos : (i64, i64, i64) = (count_from + 1000, count_from + 2000, count_from + 3000);
    println!("{:?}", arr);
    println!("{:?} {:?} {:?}", arr[mathmodulo(fin_pos.0, len)], arr[mathmodulo(fin_pos.1, len)], arr[mathmodulo(fin_pos.2, len)]);
    println!("{}", arr[mathmodulo(fin_pos.0, len)].0 + arr[mathmodulo(fin_pos.1, len)].0 + arr[mathmodulo(fin_pos.2, len)].0);
}