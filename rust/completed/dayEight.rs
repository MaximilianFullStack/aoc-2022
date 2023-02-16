use std::fs;

fn visi(grid: String) -> usize {
    let mut column: Vec<Vec<u8>> = vec![vec![]];
    let mut row: Vec<Vec<u8>> = vec![vec![]];
    grid.lines().enumerate().for_each(|(i, l)| {
        row.push(Vec::new());
        for (idx, c) in l.chars().enumerate() {
            if i == 0 {
            column.push(Vec::new());
            }
            column[idx].push(c.to_string().parse::<u8>().unwrap());
            row[i].push(c.to_string().parse::<u8>().unwrap());
    }});
    row.pop();
    column.pop();

    let mut scores: Vec<usize> = vec![];
    for r in 0..row.len() {
        row[r].iter().enumerate().for_each(|(c, _)| {
            let score = check_vis(row[r].clone(), r, column[c].clone(), c);
            scores.push(score);
        });
    }
    scores.sort_by(|a,b| b.cmp(a));
    scores[0]
}

fn check_vis(row: Vec<u8>, row_idx: usize, col: Vec<u8>, col_idx: usize) -> usize {
    let mut sm_tree: usize = 0;
    let (mut left, mut right, mut top, mut bottom) = (0,0,0,0);
    if col_idx != 0 {
        for i in 0..row.len() { 
            if row[col_idx] > row[i] {
                sm_tree += 1;
            } else {
                sm_tree = 0;
                if row[col_idx] == row[i] {
                    sm_tree += 1;
                }
            }
            if i == col_idx-1 {
                left = sm_tree;
            }
        }
    }
    sm_tree = 0;
    if col_idx != row.len()-1 {
        for i in 0..row.len() { 
            if i > col_idx {
                if row[col_idx] > row[i] {
                    sm_tree += 1;
                } else {
                    sm_tree += 1;
                    right = sm_tree;
                    break;
                }
                if i == row.len()-1 {
                    right = sm_tree;
                } 
            }
        }
    }
    sm_tree = 0;
    if row_idx != 0 {
        for i in 0..col.len() { 
            if col[row_idx] > col[i] {
                sm_tree += 1;
            } else {
                sm_tree = 0;
                if col[row_idx] == col[i] {
                    sm_tree += 1;
                }
            }
            if i == row_idx-1 {
                top = sm_tree;
            }
        }
    }
    sm_tree = 0;
    if row_idx != col.len()-1 {
        for i in 0..col.len() { 
            if i > row_idx {
                if col[row_idx] > col[i]  {
                    sm_tree += 1;
                } else {
                    sm_tree += 1;
                    bottom = sm_tree;
                    break;
                }
                if i == col.len()-1 {
                    bottom = sm_tree;
                } 
            }
        }
    }
    left * right * top * bottom
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    println!("{}", visi(contents));
}