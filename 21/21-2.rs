use std::collections::HashSet;
use std::collections::HashMap;
use std::str::Split;

const HUMAN_NAME: &str = "humn";

fn get_pair<'a>(mut line: Split<'a, &str>) -> (String, String) {
    (line.next().unwrap().to_string(), line.next().unwrap().to_string())
}

fn get_root_eq(monkeys: &HashMap<String, String>) -> String {
    let mut root_split = monkeys["root"].split(" ").collect::<Vec<_>>();
    root_split[1] = "=";
    root_split.join(" ")
}

fn get_equation(monkeys: &HashMap<String, String>) -> String{
    let mut equation = get_root_eq(&monkeys);

    loop {
        let mut changed = false;
        for entity in equation.clone().split(" ") {
            if entity.chars().all(char::is_alphabetic) && !entity.is_empty() && entity != HUMAN_NAME{
                println!("EQ: {:?} ENT: {:?}", equation, entity);
                println!("EQ: {:?} ENT: {:?} VAL: {:?}", equation, entity, monkeys[entity]);
                equation = equation.replace(entity, & if monkeys[entity].chars().all(char::is_numeric) { monkeys[entity].clone() } else { " ( ".to_owned() + &monkeys[entity] + " ) " });
                changed = true;
            }
        }
        if !changed {break}
    }

    equation
}

fn clean(mut s: String) -> String {
    while s != s.replace("  ", " ") {
        s = s.replace("  ", " ");
    }
    s.trim().to_string()
}

fn split_eq(eq: String) -> (String, String) {
    let eq_split = eq.split(" = ");
    let (left_side, right_side) = get_pair(eq_split);
    if !left_side.contains(HUMAN_NAME) {
        (right_side, left_side)
    } else {
        (left_side, right_side)
    }
}

fn unwrap_brackets(eq: &String) -> String {
    let mut eqv = eq.trim().chars().collect::<Vec<_>>();
    while *eqv.first().unwrap() == '(' && *eqv.last().unwrap() == ')' {
        //println!("cur_crop: {:?}", eqv.iter().collect::<String>());
        let mut bracket_count = 0;
        for c in eqv[..eqv.len()-1].iter() { // check if init bracket never closes
            if *c == '(' {
                bracket_count += 1;
            } else if *c == ')' {
                bracket_count -= 1;
            } if bracket_count == 0 {
                // our job is done
                return eqv.into_iter().collect();
            }
        }
        eqv = eqv[2..eqv.len()-2].to_vec();
        //println!("new: {:?}", eqv.iter().collect::<String>());
    }
    eqv.into_iter().collect()
}

fn split_by_operation(eq: &String) -> (String, char, String) {
    let operations: HashSet<char> = HashSet::from(['+', '-', '*', '/']);
    let eq_unwrapped = unwrap_brackets(eq);
    println!("SPLITTING: {:?}", eq_unwrapped);
    let mut bracket_count = 0;
    for (i, c) in eq_unwrapped.chars().enumerate() {
        if c == '(' {
            bracket_count += 1;
        } else if c == ')' {
            bracket_count -= 1;
        } else if operations.contains(&c) && bracket_count == 0 {
            println!("LEFT: {:?} OP: {:?} RIGHT: {:?}", eq_unwrapped[..i].to_string(), c, eq_unwrapped[i+1..].to_string());
            return (clean(eq_unwrapped[..i].to_string()), c, clean(eq_unwrapped[i+1..].to_string()));
        }
    }
    unreachable!();
}

fn main() {
    let monkeys: HashMap<String, String> = include_str!("../input/21.txt").lines().map(|line| get_pair(line.split(": "))).collect();
    let equation = clean(get_equation(&monkeys));
    println!("EQ: {:?}", equation);
    let (mut left_side, mut right_side) = split_eq(equation); // human is always on the left due to how this function works
    println!("LEFT_SIDE: {:?} RIGHT_SIDE: {:?}", left_side, right_side);

    // transfer left side to the right
    let op_inverse: HashMap<char, char> = HashMap::from([('+', '-'), ('-', '+'), ('*', '/'), ('/', '*')]);
    while left_side != HUMAN_NAME {
        println!("LOOPING; LEFT_SIDE: {:?} RIGHT_SIDE: {:?}", left_side, right_side);
        let (left, op, right) = split_by_operation(&left_side);
        if left.contains(HUMAN_NAME) {
            (left_side, right_side) = (left, " ( ".to_owned() + &right_side + " ) " + &op_inverse[&op].to_string() + " ( " + &right + " ) ");
            println!("CHANGED(1); LEFT_SIDE: {:?} RIGHT_SIDE: {:?}", left_side, right_side);
        }
        else if right.contains(HUMAN_NAME) {
            (left_side, right_side) = (right, " ( ".to_owned() + &left + " ) " + &op_inverse[&op].to_string() + " ( " + &right_side + " ) ");
            println!("CHANGED(2); LEFT_SIDE: {:?} RIGHT_SIDE: {:?}", left_side, right_side);
        }
    }
    println!("LEFT: {:?} RIGHT: {:?}", left_side, right_side);

    // evaluate right side

    println!("{:?}", clean(right_side));
}