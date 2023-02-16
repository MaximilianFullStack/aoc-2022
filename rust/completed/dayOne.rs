use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn get_top_cal(_cals: &[i32]) -> usize {
    let mut added: Vec<i32> = Vec::new();
    let mut added_index: usize = 0;
    for i in 0.._cals.len() - 1 {
        if i > 0 && _cals[i] == 0 {
            added_index += 1;
        } else {
            added.push(0);
            added[added_index] += _cals[i];
        }
    }
    added.sort_by(|a,b| b.cmp(a));
    (added[0] + added[1] + added[2]).try_into().unwrap()
}

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    let mut v: Vec<i32> = Vec::new();
    for line in br.lines() {
        let n = line?;
        if n != "" {
            v.push(n.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        } else {
            v.push(0);
        }
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let arr: Vec<i32> = read(File::open("./input.txt")?)?;
    println!("{}", get_top_cal(&arr));
    Ok(())
}

