fn main() {
    let input = include_str!("./input.txt");

    let splitter = input.split("\n\n");
    //println!("{:?}", splitter);

    let input_list: Vec<&str> = splitter.collect();

    //println!("{:?}", input_list[0]);

    let mut max_food = 0;
    for deer in input_list {
        let food = deer.lines();
        //println!("{:?}",food);
        let calories = food.map(|food_item| food_item.parse::<i32>().unwrap()).sum();
        if calories > max_food {
            max_food = calories;
        }
    }
    println!("{:?}",max_food);
}