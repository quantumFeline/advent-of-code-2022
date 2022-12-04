fn main() {

    let mut sum = 0;
    for line in include_str!("../input/4.txt").lines() {
        let assignments: Vec<&str> = line.split(",").collect();
        let elf1: Vec<i32> = assignments[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = assignments[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        if elf1[0] >= elf2[0] && elf1[1] <= elf2[1] || elf1[0] <= elf2[0] && elf1[1] >= elf2[1] {
            sum+=1;
        }
    }

    println!("{:?}",sum);
}