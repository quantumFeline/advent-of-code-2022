use std::collections::HashMap;

struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: HashMap<&'a str, Vec<&'a str>>,
    flow: HashMap<&'a str, i32>,
}

impl Graph<'_> {
    fn new(data: Vec<(&'static str, i32, Vec<&'static str>)>) -> Self{
        let mut nodes = vec![];
        let mut edges = HashMap::new();
        let mut flow = HashMap::new();
        for datum in data {
            nodes.push(datum.0);
            flow.insert(datum.0, datum.1);
            edges.insert(datum.0, datum.2);
            edges.get_mut(datum.0).unwrap().push(datum.0); // corridor to itself
        }
        Graph{nodes, edges, flow}
    }
}

fn read() -> Vec<(&'static str, i32, Vec<&'static str>)> {
    let input = include_str!("../input/16.txt").lines();

    let mut data = vec![];

    for line in input {
        let valve = &line[6..8];
        let flow = line.split_once('=').unwrap().1.split_once(';').unwrap().0.parse().unwrap();
        let adjacent: Vec<&str> = line.split_once("to valve").unwrap().1[1..].trim_start().split(", ").collect();

        data.push((valve, flow, adjacent));
    }
    data
}

fn main() {
    let data = read();
    let g = Graph::new(data);

    const VALVE_UNREACHABLE: i32 = -1_000_000;
    const SECONDS: usize = 30 + 1;
    let empty_vec: Vec<&str> = vec![];

    // path search
    let mut dp: Vec<HashMap<&str, i32>> = vec![HashMap::new(); SECONDS];
    let mut dp_open_valves: Vec<HashMap<&str, Vec<&str>>> = vec![HashMap::new(); SECONDS];
    // value = max active flow rate at i-th second, if we're standing in front of j-th valve
    for valve in g.nodes.iter() {
        if *valve == "AA" {
            dp[0].insert(valve, 0);
        } else {
            dp[0].insert(valve, VALVE_UNREACHABLE);
        }
    }

    println!("{:?} \n {:?}", g.flow, g.edges);

    for i in 1..SECONDS {
        for valve in g.nodes.iter() {
            let mut best_flow = if dp[i].get(valve).is_some() {*dp[i].get(valve).unwrap()} else {VALVE_UNREACHABLE};
            for prev_valve in g.edges.get(valve).unwrap() {
                let get_there_flow = *dp[i-1].get(prev_valve).unwrap();
                let get_there_and_open_valve_flow = if i != 1 {*dp[i-2].get(prev_valve).unwrap() + g.flow.get(valve).unwrap()} else {VALVE_UNREACHABLE};
                if best_flow < get_there_flow { // just get there from neighbouring point by i-th second
                    best_flow = get_there_flow;
                    let dp_prev = dp_open_valves[i].get(prev_valve).unwrap_or_else(|| &empty_vec).clone();
                    dp_open_valves[i].insert(valve, dp_prev);
                    println!("Walking from {} to {}", prev_valve, valve);
                } if best_flow < get_there_and_open_valve_flow && !dp_open_valves[i].get(valve).unwrap_or_else(|| &empty_vec).contains(valve) {
                    // get there AND open the valve by that time
                    best_flow = get_there_and_open_valve_flow;
                    let dp_prev = dp_open_valves[i].get(prev_valve).unwrap_or_else(|| &empty_vec).clone();
                    dp_open_valves[i].insert(valve, dp_prev);
                    dp_open_valves[i].get_mut(valve).map(|val| val.push(valve));
                    println!("Opening from {} to {}", prev_valve, valve)
                }
                dp[i].insert(valve, best_flow);
                println!("{} {:?}", i, dp[i]);
            }
        }
    }
    for i in 0..SECONDS {
        println!("{} {:?}", i, dp[i]);
    }
}
