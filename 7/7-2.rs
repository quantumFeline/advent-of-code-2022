use std::collections::HashMap;

fn to_tuple(x: &str, current_folder: &str, dir_sizes: &HashMap<String, i32>) -> (Option<i32>, String) {
    let x_vec = x.split(" ").collect::<Vec<_>>();
    let mut full_path = String::from(current_folder);
    full_path.push('/');
    full_path.push_str(x_vec[1]);

    let size: Option<i32> =
    if x_vec[0] == "dir" {
        dir_sizes.get(&full_path).copied() // may be None, it's fine
    } else {
        x_vec[0].parse::<i32>().ok().as_ref().copied()
    };

    (size, full_path)
}

fn calculate_folder_size(files: &Vec<(Option<i32>, String)>) -> i32 {
    let mut folder_size = 0;
    for file in files {
        println!("ls file: {:?} {:?}", file.0, file.1);
        match file.0 {
            Some(size) => folder_size += size,
            None => {folder_size = -1; break;}
        }
    }
    folder_size
}

fn collect_file_sizes(folder_ls: &HashMap<String, &str>, current_folder: &str, dir_sizes: &HashMap<String, i32>) -> Vec<(Option<i32>, String)>{
    folder_ls.get(current_folder).unwrap().split("\n").map(|x| to_tuple(x, &current_folder, &dir_sizes)).collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../input/7.txt");

    let input: Vec<_> = input[9..input.len()].split("\n$ ").collect();

    let mut position: Vec<&str> = vec!["/"];
    let mut folder_sizes: HashMap<String, i32> = HashMap::new();
    let mut folder_ls: HashMap<String, &str> = HashMap::new();

    for command in input {
        if &command[0..2] == "cd" {
            match &command[3..command.len()] {
                "/" => {position.clear(); position.push("/");},
                ".." => {let _ = position.pop();},
                inner_folder => position.push(inner_folder)
            }
        } else if &command[0..2] == "ls" {
            folder_ls.insert(position.join("/"), &command[3..command.len()]);
        }
        let current_folder = position.join("/");
        println!("current folder: {}", &current_folder);

        if folder_ls.contains_key(&current_folder) {
            let files = collect_file_sizes(&folder_ls, &current_folder, &folder_sizes);
            let folder_size = calculate_folder_size(&files);
            if folder_size != -1 {
                folder_sizes.insert(current_folder, folder_size);
            }
        }
    }

    while position.len() != 1 { // just the root
        position.pop();
        let current_folder = position.join("/");
        let files = collect_file_sizes(&folder_ls, &current_folder, &folder_sizes);
        let folder_size = calculate_folder_size(&files);
        folder_sizes.insert(current_folder, folder_size);
    }
    //println!("folder sizes: {:?}", folder_sizes);

    let total_disk_space = 70000000;
    let space_to_free = 30000000 - (total_disk_space - folder_sizes.get("/").unwrap());
    let mut best_choice_size = total_disk_space;
    println!("{}", space_to_free);
    for (_folder_name, folder_size) in folder_sizes {
            if folder_size >= space_to_free && folder_size < best_choice_size {
                best_choice_size = folder_size;
            }
    }

    println!("{}", best_choice_size);
}