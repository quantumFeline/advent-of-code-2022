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
                equation = equation.replace(entity,&(" ( ".to_owned() + &monkeys[entity] + " ) "));
                changed = true;
            }
        }
        if !changed {break}
    }

    equation
}

fn clean_double_spaces(mut s: String) -> String {
    while s != s.replace("  ", " ") {
        s = s.replace("  ", " ");
    }
    s
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

fn main() {
    let monkeys: HashMap<String, String> = include_str!("../input/21.txt").lines().map(|line| get_pair(line.split(": "))).collect();
    let equation = clean_double_spaces(get_equation(&monkeys));
    println!("EQ: {:?}", equation);
    let (left_side, mut right_side) = split_eq(equation); // human is always on the left due to how this function works
    println!("LEFT: {:?} RIGHT: {:?}", left_side, right_side);

    let mut left_side_split = left_side.split(" ");
    let mut get_next_two = || -> (String, String) {
        (left_side_split.next().unwrap_or("").to_string(), left_side_split.next().unwrap_or("").to_string())
    };

    let (mut token, mut op) = get_next_two();
    // left to human in eq
    while token.as_str() != HUMAN_NAME {
        right_side = match op.as_str() {
            "+" => {right_side.to_owned() + " - " + &token},
            "-" => {token.to_owned() + " - (" + &right_side + ")"},
            "*" => {right_side.to_owned() + " / " + &token},
            "/" => {token.to_owned() + " / (" + &right_side + ")"},
            "" => {right_side},
            _ => unreachable!()
        };
        println!("TOKEN: {:?} OP: {:?} EQ: {:?}", token, op, right_side);
        (token, op) = get_next_two();
    }

    // right to human
    loop {
        right_side = match op.as_str() {
            "+" => {right_side.to_owned() + " - " + &token},
            "-" => {token.to_owned() + " + (" + &right_side + ")"},
            "*" => {right_side.to_owned() + " / " + &token},
            "/" => {token.to_owned() + " * (" + &right_side + ")"},
            "" => {right_side},
            _ => unreachable!()
        };

        println!("TOKEN: {:?} OP: {:?} EQ: {:?}", token, op, right_side);
        let (token, op) = get_next_two();
        if token == "" || op == "" {
            break;
        }
    }
    println!("{:?}", right_side);
}