use std::collections::HashMap;


fn to_tuple(x: &str, position: &Vec<&str>, dir_sizes: &HashMap<String, i32>) -> (Option<i32>, String) {
    let x_vec = x.split(" ").collect::<Vec<_>>();
    let full_path = position.join("/")+ "/" + x_vec[1];

    let size: Option<i32> =
    if x_vec[0] == "dir" {
        dir_sizes.get(&full_path).copied() // may be None, it's fine
    } else {
        x_vec[0].parse::<i32>().ok().as_ref().copied()
    };

    //println!("x_vec: {:?} size {:?} full_path {:?}", x_vec, size, full_path);

    (size, full_path)
}

fn main() {
    let input = include_str!("../input/7.txt");

    let input: Vec<_> = input[9..input.len()].split("\n$ ").collect();
    //println!("{:?}", input);

    let mut position: Vec<&str> = vec!["/"];
    let mut folder_sizes: HashMap<String, i32> = HashMap::new();
    let mut folder_ls: HashMap<String, &str> = HashMap::new();
    let mut small_folder_sum = 0;

    for command in input {
        if &command[0..2] == "cd" {
            match &command[3..command.len()] {
                "/" => position.clear(),
                ".." => {
                    let _last_folder = position.pop().unwrap();
                    //println!("last {:?}", last_folder);
                },
                inner_folder => {
                    //println!("inner {:?}", inner_folder);
                    position.push(inner_folder);
                }
            }
        } else if &command[0..2] == "ls" {
            folder_ls.insert(position.join("/"), &command[3..command.len()]);
        }
        let current_folder: String = String::from(position.join("/"));
        //println!("current folder: {}", &current_folder);

        if folder_ls.contains_key(&current_folder) {
            let files = folder_ls.get(&current_folder).unwrap().split("\n").map(|x| to_tuple(x, &position, &folder_sizes)).collect::<Vec<_>>();

            let mut folder_size = 0;
            for file in files {
                //println!("ls file: {:?} {:?}", file.0, file.1);
                match file.0 {
                    Some(size) => folder_size += size,
                    None => {folder_size = -1; break;}
                }
            }
            if folder_size != -1 {
                folder_sizes.insert(current_folder, folder_size);
                //println!("folder sizes: {:?}", folder_sizes);
                if folder_size <= 100000 {
                    small_folder_sum += folder_size;
                    //println!("small folder sum incremented to {}", small_folder_sum);
                }
            }
        }
    }

    println!("{}", small_folder_sum);
}