fn main() {
    let input = include_str!("./input.txt");

    let splitter = input.split("\n\n");
    //println!("{:?}", splitter);

    let input_list: Vec<&str> = splitter.collect();

    //println!("{:?}", input_list[0]);

    let mut max_food = vec![0,0,0];
    for deer in input_list {
        let food = deer.lines();
        //println!("{:?}",food);
        let mut calories = 0;
        for food_item in food {
            calories += food_item.parse::<i32>().unwrap();
            //println!("{:?}", food_item.parse::<i32>().unwrap());
        }
        if calories > max_food[0] {
            max_food[2] = max_food[1];
            max_food[1] = max_food[0];
            max_food[0] = calories;
        } else if calories > max_food[1] {
            max_food[2] = max_food[1];
            max_food[1] = calories;
        } else if calories > max_food[2] {
            max_food[2] = calories;
        }
    }
    println!("{:?}, ({:?})",max_food.iter().sum::<i32>(), max_food);
}