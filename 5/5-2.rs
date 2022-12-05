fn main() {

    //let mut objects: Vec<Vec<char>> = vec![vec!['Z','N'],vec!['M','C','D'],vec!['P']];
    let mut objects: Vec<Vec<char>> = vec![];
    objects.resize(1000, vec![]);
    let input_lines = include_str!("../input/5.txt").lines();

    let mut moving_stage: bool = false;

    for line in input_lines {

        if line.len() == 0 {
            moving_stage = true;
            objects.iter_mut().for_each(|stack| stack.reverse());
            continue;
        }

        if !moving_stage {
            for (i, symbol) in line.chars().enumerate() {
                if symbol as u32 >= 'A' as u32 && symbol as u32 <= 'Z' as u32 {
                    objects[i/4].push(symbol);
                }
            }
        } else {
            let k1: Vec<&str> = line[5..line.len()].split(" ").collect();
            let n = k1[0].parse::<usize>().unwrap();
            let from = k1[2].parse::<usize>().unwrap() - 1;
            let to = k1[4].parse::<usize>().unwrap() - 1;

            let mut middle_vec: Vec<char> = vec![];
            for _i in 0..n {
                let element = objects[from].pop().unwrap();
                middle_vec.push(element);
            }
            for _i in 0..n {
                let element = middle_vec.pop().unwrap();
                objects[to].push(element);
            }
        }
    }

    //println!("{:?}",objects.iter().map(|x| x.last()).filter(|&el| el.is_some()).map(|el| el.as_ref().unwrap()).collect::<Vec<_>>());
    println!("{}", objects.iter().map(|x| x.last()).filter(|&el| el.is_some()).map(|el| el.unwrap()).collect::<String>());
}