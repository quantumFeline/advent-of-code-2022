fn main() {
    let input = include_str!("../input/6.txt");
    for (i, symbol) in input.chars().enumerate() {
        if i < 4 { continue; }
        let p = input.as_bytes()[i-1] as char;
        let pp = input.as_bytes()[i-2] as char;
        let ppp = input.as_bytes()[i-3] as char;
        if symbol != p && symbol != pp && symbol != ppp && p != pp && p != ppp && pp != ppp{
            println!("Ans: {}", i+1);
            break;
        }
    }
}