use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum MonkeyState<'a> {
    Waiting {monkey1: &'a str, monkey2: &'a str, operation: char},
    Ready(i64)
}

fn main() {
    let input = include_str!("../input/21.txt").lines().map(|line| {let mut l = line.split(": "); (l.next().unwrap(), l.next().unwrap())});
    let mut monkeys: HashMap<&str, MonkeyState> = HashMap::new();
    let mut monke_keys = vec![];
    let mut monkeys_ready = 0;
    for (monkey, operation) in input {
        let state = match operation.parse::<i64>() {
            Ok(v) => {
                monkeys_ready += 1;
                MonkeyState::Ready(v)
            }
            Err(_e) => {
                let mut o_split = operation.split(" ");
                let m1 = o_split.next().unwrap();
                let op = o_split.next().unwrap().chars().next().unwrap();
                let m2 = o_split.next().unwrap();
                MonkeyState::Waiting {monkey1: m1, monkey2: m2, operation: op}
            }
        };
        println!("{:?} {:?} {:?}", monkey, operation, state);
        monke_keys.push(monkey);
        monkeys.insert(monkey, state);
    }
    while monkeys_ready < monkeys.len() {
        println!("ready: {:?}", monkeys_ready);
        for monke in monke_keys.iter() {
            let mut res = None;
            let state = monkeys.get(*monke).unwrap();
            match state {
                MonkeyState::Ready(_v) => {continue;},
                MonkeyState::Waiting {monkey1: m1, monkey2: m2, operation: op} => {
                    if let MonkeyState::Ready(m1v) = monkeys[m1] {
                        if let MonkeyState::Ready(m2v) = monkeys[m2] {
                            res = Some(match op {
                                '+' => m1v + m2v,
                                '-' => m1v - m2v,
                                '*' => m1v * m2v,
                                '/' => m1v / m2v,
                                _ => panic!()
                            });
                        }
                    }
                }
            };
            if let Some(val) = res {
                monkeys.insert(monke, MonkeyState::Ready(val));
                monkeys_ready += 1;
            }
        }
    }
    println!("{:?}", monkeys.get("root").unwrap());
}