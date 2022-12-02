fn index(player_move: char) -> usize {
    let mut ans: usize = 0;
    if player_move == 'B' || player_move == 'Y' {
        ans = 1;
    } else if player_move == 'C' ||player_move == 'Z' {
        ans = 2;
    }
    ans
}

fn main() {

    let score_table = vec![[4, 8, 3], [1, 5, 9], [7, 2, 6]];

    let input: Vec<&str> = include_str!("./input/2.txt").lines().collect();
    let mut score: i32 = 0;

    for line in input {
        let moves: Vec<&str> = line.split(" ").collect();
        score += score_table[index(moves[0usize].chars().nth(0).unwrap())][index(moves[1usize].chars().nth(0).unwrap())];
        println!("{:?} {}",moves, score);
    }

    println!("{:?}",score);
}