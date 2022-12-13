use std::cmp::Ordering;

fn is_list (s: &str) -> bool {
    s.chars().next() == Some('[')
}

fn to_list(s: &str) -> Vec<&str> {
    let mut v: Vec<&str> = vec![];
    let mut inner = 0;
    let mut last_s = 1;
    for (i,c) in s.chars().enumerate() {
        if c == '[' {inner += 1}
        else if c == ']' {inner -= 1}
        else if c == ',' && inner == 1 {
            v.push(&s[last_s..i]);
            last_s = i+1;
        }
    }
    if last_s < s.len()-1 {v.push(&s[last_s..s.len()-1])}; // last element
    //println!("list: {:?}", v);
    v
}

fn cmp_list_list(s1: &str, s2: &str) -> Ordering {
    let left_list = to_list(s1);
    let right_list = to_list(s2);
    let mut l_iter = left_list.iter();
    let mut r_iter = right_list.iter();
    //let list_pairs = (0..std::Ordering:::min(left_list.len(), right_list.len())).map(|i| (left_list[i], right_list[i]));
    loop {
        let l = l_iter.next();
        let r = r_iter.next();
        //println!("cmp_list: {:?} with {:?}", l, r);

        if l == None && r == None {return Ordering::Equal}
        else if l == None {return Ordering::Less}
        else if r == None {return Ordering::Greater};

        let cur_cmp = cmp(l.unwrap(), r.unwrap());
        if cur_cmp != Ordering::Equal {return cur_cmp}
    }
}

fn cmp (s1: &str, s2: &str) -> Ordering {
    //println!("cmp {:?} with {:?}", s1, s2);
    if is_list(s1) && is_list(s2) {
        return cmp_list_list(s1, s2);
    }
    else if is_list(s1) {
        let s2_list = "[".to_owned() + s2 + "]";
        return cmp(s1, &s2_list);
    }
    else if is_list(s2) {
        let s1_list = "[".to_owned() + s1 + "]";
        return cmp(&s1_list, s2);
    } else {
        //println!("i1={:?}, i2={:?}", s1, s2);
        let i1 = s1.parse::<i32>().unwrap();
        let i2 = s2.parse::<i32>().unwrap();
        return if i1 < i2 {Ordering::Less} else if i1 == i2 {Ordering::Equal} else {Ordering::Greater};
    }
}

fn main() {
    let mut input = include_str!("../input/13.txt").split("\n").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    input.append(&mut vec!["[[2]]", "[[6]]"]);
    input.sort_by(|a, b| cmp(a, b));

    let div1 = input.iter().position(|&r| r == "[[2]]").unwrap();
    let div2 = input.iter().position(|&r| r == "[[6]]").unwrap();
    println!("{}", (div1+1)*(div2+1));
}