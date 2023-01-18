use std::fs;
use std::collections::HashSet;

fn starter(data: String) -> usize {
    let mut end: usize = 0;
    data.chars().enumerate().collect::<Vec<(usize, char)>>().windows(14)
        .for_each(|l| {
            let l2 = l.into_iter().map(|i| i.1).collect::<Vec<char>>();
            let h: HashSet<&char> = HashSet::from_iter(l2.iter());
            if h.len() > 13 && end == 0 {
                end = l[13].0 +1;
            }
        });
    end
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", starter(contents));
}