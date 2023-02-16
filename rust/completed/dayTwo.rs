use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn skiz(opp: &[u8], us: &[u8]) -> usize {
    let mut score: u32 = 0;
    for i in 0..opp.len() {
        if opp[i] == us[i] {
            score += 3;
        } else {
            match opp[i] {
                1 if us[i] == 2 => score+=6,
                2 if us[i] == 3 => score+=6,
                3 if us[i] == 1 => score+=6,
                _ => (),
            }
        }     
        match us[i] {
            1 => score+=1,
            2 => score+=2,
            3 => score+=3,
            _ => panic!(),
        }
    }
    score.try_into().unwrap()
}

fn determine_move(opp: &[u8], us: &[u8]) -> Vec<u8> {
    let mut new: Vec<u8> = Vec::new();
    for i in 0..opp.len() {
        if us[i] == 2 {
            new.push(opp[i]);
        }
        match opp[i] {
            1 if us[i] == 1 => new.push(3),
            1 if us[i] == 3 => new.push(2),
            2 if us[i] == 1 => new.push(1),
            2 if us[i] == 3 => new.push(3),
            3 if us[i] == 1 => new.push(2),
            3 if us[i] == 3 => new.push(1),
            _ => (),
        }
    }
    new
}

fn read<R: Read>(io: R) -> Result<(Vec<u8>,Vec<u8>), Error> {
    let br = BufReader::new(io);
    let mut l: Vec<u8> = Vec::new();
    let mut r: Vec<u8> = Vec::new();
    for line in br.lines() {
        let n = line?;
        for sub in n.split_whitespace() {
            match sub {
                "A" => l.push(1),
                "B" => l.push(2),
                "C" => l.push(3),
                "X" => r.push(1),
                "Y" => r.push(2),
                "Z" => r.push(3),
                &_ => panic!()
            }
        }
    }
    Ok((l,r))
}

fn main () -> Result<(), Error> {
    let (o, u) = read(File::open("./input.txt")?)?;
    let u2: Vec<u8> = determine_move(&o, &u);
    println!("{}", skiz(&o, &u2));
    Ok(())
}

// A & X == Rock || 1
// B & Y == Paper || 2
// C & Z == Scissors || 3