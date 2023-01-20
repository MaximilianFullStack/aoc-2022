use std::fs;
use std::collections::HashMap;

fn map_size(command: String) -> usize {
    let mut dir_size: HashMap<String, usize> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    command.lines().for_each(|line| {if line.find("$ cd ") != None {
        if line.find("$ cd ..") == None {
            let mut current = line.chars().rev().take_while(|c| (c.clone() as u8) != 32).collect::<String>().chars().rev().collect::<String>();
            for dir in 0..path.len() {
                current.push_str(&path[dir]);
            }
            path.push(current.clone().trim().to_string());
        } else {
            path.pop();
        }
    } else if (line.as_bytes()[0]) != 36 {
        if line.find("dir ") == None {
            let num: usize = line.chars().take_while(|c| (c.clone() as u8) != 32).collect::<String>().parse().unwrap();
            for dir in 0..path.len() {
                *dir_size.entry(path[dir].clone()).or_insert(0) += num;
            }
        }       
    }});
    return part_2(dir_size);
}

fn part_1(map: HashMap<String,usize>) -> usize {
    return map.values().filter(|s| s < &&(100_000 as usize)).sum::<usize>();
}

fn part_2(map: HashMap<String, usize>) -> usize {
    let mut v: Vec<usize> = Vec::new();
    v.extend(map.values().filter(|s| s > &&(30_000_000 - ((70_000_000 as usize) - map.values().max().unwrap()))).map(|n| n));
    v.sort_by(|a,b| a.cmp(b));
    v[0]
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", map_size(contents));
}