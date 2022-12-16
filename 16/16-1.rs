use std::collections::HashMap;

fn main() {
    const SECONDS: usize = 30;

    let s1 = "Valve ";
    let s2 = " has flow rate=";
    let s3 = "; tunnels lead to valves ";
    let s3_var = "; tunnel leads to valve ";
    let input = include_str!("../input/16.txt").lines();

    let mut valve_flow = HashMap::new();
    let mut tunnels = HashMap::new();

    for line in input {
        let (valve, valve_info) = [line[s1.len()..].split(s2)].map(|mut x| (x.next().unwrap(), x.next().unwrap()))[0];
        let (info1, info2) =
        if valve_info.split(s3).collect::<Vec<_>>().len() == 2 {
            [valve_info.split(s3)].map(|mut x| (x.next().unwrap(), x.next().unwrap()))[0]
        } else {
            [valve_info.split(s3_var)].map(|mut x| (x.next().unwrap(), x.next().unwrap()))[0]
        };
        let flow_rate = info1.parse::<u32>().unwrap();
        let valve_tunnels = info2.split(", ").collect::<Vec<_>>();
        println!("{:?} {:?} {:?}", valve, flow_rate, valve_tunnels);

        valve_flow.insert(valve, flow_rate);
        tunnels.insert(valve, valve_tunnels);
    }

    const VALVE_UNREACHABLE: i32 = -1_000_000;

    // path search
    let mut dp: Vec<HashMap<&str, i32>> = vec![HashMap::new(); SECONDS];
    let mut dp_open_valves: Vec<HashMap<&str, Vec<&str>>> = vec![HashMap::new(); SECONDS];
    // value = max active flow rate at i-th second, if we're standing in front of j-th valve

    for valve in valve_flow.keys() {
        if *valve == "AA" {
            dp[0].insert(valve, 0);
        } else {
            dp[0].insert(valve, VALVE_UNREACHABLE);
        }
    }

    for i in 1..SECONDS {
        for valve in valve_flow.keys() {
            let mut best_flow = VALVE_UNREACHABLE;
            for prev_valve in tunnels.get(valve).unwrap() {
                let get_there_flow = dp[i-1].get(prev_valve).unwrap();
                let get_there_and_open_valve_flow = if i != 1 {dp[i-2].get(prev_valve).unwrap() + valve_flow.get(valve).unwrap()} else {VALVE_UNREACHABLE};
                if best_flow < get_there_flow { // just get there from neighbouring point by i-th second
                    best_flow = get_there_flow;
                } if best_flow < get_there_and_open_valve_flow { // get there AND open the valve by that time
                    best_flow = get_there_and_open_valve_flow;
                    
                }
            }
        }
    }

    println!("{:?} \n {:?}", valve_flow, tunnels);
}
