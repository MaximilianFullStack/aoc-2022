use std::fs;

fn stack(data: String) -> String {
    let mut boxes:Vec<Vec<char>> = vec![vec![]; 9];
    data.lines().take(8).for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            let b = c as u8;
            if b < 91 && b > 64 {
                boxes[i/4].push(c);
            }
        }
    });

    data.lines().skip(10).for_each(|l| {
        let mut s = (l.as_bytes()[5] as char).to_string();
        s.push_str(&(l.as_bytes()[6] as char).to_string());
        let mut f = (l.as_bytes()[12] as char).to_string();
        f.push_str(&(l.as_bytes()[13] as char).to_string());
        let mut t = (l.as_bytes()[17] as char).to_string();
        if l.len() > 18 {t.push_str(&(l.as_bytes()[18] as char).to_string());}

        let m: Vec<char> = boxes[f.trim().parse::<usize>().unwrap() - 1][0..s.trim().parse::<usize>().unwrap()].to_vec();
        // let rev_m = m.into_iter().rev().collect::<Vec<char>>(); 
        boxes[f.trim().parse::<usize>().unwrap() - 1].drain(0..s.trim().parse::<usize>().unwrap());
        boxes[t.trim().parse::<usize>().unwrap() - 1].splice(..0, m); // input rev_m instead of m for part 1 anwser
    });

    let mut total: String = String::new();
    for i in 0..boxes.len() {
        total.push_str(&boxes[i][0].to_string());
    }
    total
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", stack(contents));
}