#[derive(Debug)]
struct Monkey<'a> {
    operation: &'a str,
    div_test: i32,
    true_catcher: usize,
    false_catcher: usize,
    n_inspection: usize
}

fn exec_op(old: i32, command: &str) -> i32 {
    let expr = command.replace("old", &old.to_string());
    let mut ans = 0;
    // assume only two numbers in the expr
    if expr.contains("+") {
        ans = expr.split(" + ").map(|i| i.parse::<i32>().unwrap()).sum();
    }
    else if expr.contains("*") {
        ans = expr.split(" * ").map(|i| i.parse::<i32>().unwrap()).fold(1, |acc, x| acc * x);
    }
    ans
}

fn main() {

    let input = include_str!("../input/11.txt").split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];
    let mut items: Vec<Vec<i32>> = vec![];

    for monkey in input {
        let monkey_lines: Vec<&str> = monkey.lines().collect();
        monkeys.push(Monkey {
            operation: &monkey_lines[2][19..],
            div_test: monkey_lines[3][21..].parse::<i32>().unwrap(),
            true_catcher: monkey_lines[4][29..].parse::<usize>().unwrap(),
            false_catcher: monkey_lines[5][30..].parse::<usize>().unwrap(),
            n_inspection: 0
        });

        items.push(monkey_lines[1][18..].split(", ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>());
    }
    for _i in 0..20 {
        for (mi, monkey) in monkeys.iter_mut().enumerate() {
            let mut passed_items = vec![];
            for item in &items[mi] {
                let new_worry = exec_op(*item, monkey.operation) / 3;
                let catcher = if new_worry % monkey.div_test == 0 {
                    monkey.true_catcher
                } else {
                    monkey.false_catcher
                };
                passed_items.push((catcher,new_worry));
            }
            for (catcher, worry) in passed_items {
                items[catcher].push(worry);
            }
            monkey.n_inspection += items[mi].len();
            items[mi].clear();
        }
    }
    monkeys.sort_by_key(|m| -(m.n_inspection as i32));
    println!("{:?}", monkeys);
    println!("{}", monkeys[0].n_inspection * monkeys[1].n_inspection);
}