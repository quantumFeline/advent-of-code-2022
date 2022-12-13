#[derive(Debug)]
#[derive(PartialEq)]
enum Cmp {
    Less,
    Eq,
    More
}

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

fn cmp_list_list(s1: &str, s2: &str) -> Cmp {
    let left_list = to_list(s1);
    let right_list = to_list(s2);
    let mut l_iter = left_list.iter();
    let mut r_iter = right_list.iter();
    //let list_pairs = (0..std::Cmp:::min(left_list.len(), right_list.len())).map(|i| (left_list[i], right_list[i]));
    loop {
        let l = l_iter.next();
        let r = r_iter.next();
        //println!("cmp_list: {:?} with {:?}", l, r);

        if l == None && r == None {return Cmp::Eq}
        else if l == None {return Cmp::Less}
        else if r == None {return Cmp::More};

        let cur_cmp = cmp(l.unwrap(), r.unwrap());
        if cur_cmp != Cmp::Eq {return cur_cmp}
    }
}

fn cmp (s1: &str, s2: &str) -> Cmp {
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
        return if i1 < i2 {Cmp::Less} else if i1 == i2 {Cmp::Eq} else {Cmp::More};
    }
}

fn main() {
    let input = include_str!("../input/13.txt").split("\n\n").map(|pair| {let mut s = pair.split("\n"); (s.next().unwrap(), s.next().unwrap())});

    let mut ans = 0;

    for (i, (s1, s2)) in input.enumerate() {
        println!("Input: {:?} {:?}", s1, s2);
        let comp = cmp(s1, s2);
        ans += if comp != Cmp::More {i+1} else {0};
        println!("Result: {:?}\n", comp);
    }
    println!("{}", ans);
}