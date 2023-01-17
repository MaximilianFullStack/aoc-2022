use std::fs;

// this could be optimized 
fn skeg(duels: String) -> u32 {
    let mut count: u32 = 0;
    duels.lines()
        .for_each(|line| {
            let num1 =line.chars()
            .take_while(|&ch| ch != '-')
            .collect::<String>().parse::<i32>().unwrap();
            let num2 =line.chars()
            .take_while(|&ch| ch != ',')
            .collect::<String>().chars().rev().take_while(|&ch| ch != '-').collect::<String>().chars().rev().collect::<String>().parse::<i32>().unwrap();
            let num3 =line.chars().rev()
            .take_while(|&ch| ch != ',')
            .collect::<String>().chars().rev().take_while(|&ch| ch != '-').collect::<String>().parse::<i32>().unwrap();
            let num4 =line.chars().rev()
            .take_while(|&ch| ch != '-')
            .collect::<String>().chars().rev().collect::<String>().parse::<i32>().unwrap();
            
            if part2(num1,num2,num3,num4) {
                count += 1;
            }
        });
    count
}

fn part2(num1: i32, num2: i32, num3: i32, num4: i32) -> bool {
    if (num1..=num2).contains(&num3) | (num1..=num2).contains(&num4) | (num3..=num4).contains(&num1) | (num3..=num4).contains(&num1)  {
        true
    } else {
        false
    }
}

fn part1(num1: i32, num2: i32, num3: i32, num4: i32) -> bool {
    if num1 <= num3 && num2 >= num4 || num3 <= num1 && num4 >= num2 {
        true
    } else {
        false
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", skeg(contents));
}