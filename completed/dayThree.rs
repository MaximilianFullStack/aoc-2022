use std::fs;

// Part 1
fn ruks(sacks: String) -> u32 {
    let mut count: u32 = 0;
    sacks.lines()
        .for_each(|line| 
            { let (l, r) = line.split_at(line.len() / 2); 
                for c in l.chars() { if r.find(c) != None {
                    let b = r.as_bytes()[r.find(c).unwrap()];
                    if b > 95 {
                        count += b as u32 - 96;
                        break;
                    } else {
                        count += b as u32 - 38;
                        break;
                    }
                }}});
    count
}

// Part 2
fn ruks_2(sacks: String) -> u32 {
    let mut count: u32 = 0;
    for i in 0..sacks.lines().count()/3 {
        let arr: Vec<&str> = sacks.lines().skip(i * 3).take(3).map(|line| line).collect::<Vec<&str>>();
        for c in arr[0].chars() {
            if arr[1].find(c) != None && arr[2].find(c) != None {
                let b = arr[2].as_bytes()[arr[2].find(c).unwrap()];
                if b > 95 {
                    count += b as u32 - 96;
                    break;
                } else {
                    count += b as u32 - 38; 
                    break;
                }
            }
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", ruks_2(contents));
}