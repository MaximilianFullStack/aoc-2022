use std::fs;
use std::collections::HashMap;

// first attempt, it's terrible
fn depreceated(command: String) -> usize {
    let mut dir_size: HashMap<String, usize> = HashMap::new();
    let mut parent_dirs: HashMap<String, Vec<String>> = HashMap::new();
    command.lines().enumerate().for_each(|(i, l)| {if (l.as_bytes()[0]) == 36 {
        if l.find("$ cd") != None {
            let parent: String = l.chars().rev().take_while(|c| (c.clone() as u8) != 32).collect::<String>().chars().rev().collect();
            for (idx, line) in command.lines().enumerate() {
                if idx > (i) && line.find("$ ls") == None {
                    if line.find("$ cd") == None {
                        if line.find("dir ") != None {
                            let child: String = line.chars().rev().take_while(|c| (c.clone() as u8) != 32).collect::<String>().chars().rev().collect::<String>();
                            if !parent_dirs.contains_key(&child) {
                                parent_dirs.entry(child.clone()).or_insert(Vec::new()).push(parent.clone());
                                if parent_dirs.contains_key(&parent) {
                                    let mut arr = parent_dirs.get_mut(&parent).unwrap().clone();
                                    parent_dirs.entry(child).or_insert(Vec::new()).append(&mut arr);
                                }
                            }
                        } else {
                            let num: usize = line.chars().take_while(|c| (c.clone() as u8) != 32).collect::<String>().parse().unwrap();
                            *dir_size.entry(parent.clone()).or_insert(0) += num;
                            if parent_dirs.contains_key(&parent) {
                                for dir in parent_dirs.get_mut(&parent).unwrap() {
                                    *dir_size.entry(dir.clone()).or_insert(0) += num;
                                }
                            }   
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }});
    return dir_size.values().filter(|s| s <= &&(100_000 as usize)).sum::<usize>();
}

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