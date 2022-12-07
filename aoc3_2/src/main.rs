use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let f = match File::open("aoc22_3_2_input.txt") {
        Ok(f) => {f},
        Err(_) => {return;},
    };
    let mut reader = BufReader::new(f);
    let mut total_score: usize = 0;
    let mut eof = false;
    while !eof {
        let mut group = [String::new(), String::new(), String::new()];
        
        for i in 0..=2 {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    eof = true;
                    break;
                },
                Ok(_) => {
                    group[i] = line.trim().to_owned();
                }
                Err(_) => {
                    eof = true;
                    break;
                },
            };
        }
        
        let mut found = [[false; 53];3];

        for (i, arr) in group.iter().enumerate(){
            
            for c in arr.chars() {
                found[i][char_to_priority(c)] = true;
            }
    
        }
        total_score += find_common(&found);

    }
    println!("{:?}", total_score);
}

fn char_to_priority(c : char) -> usize {
    if c.is_lowercase() {
        return (c.to_digit(36).unwrap() - 9) as usize;
    }
    (c.to_digit(36).unwrap() - 9 + 26) as usize
}

fn find_common(found : &[[bool; 53]; 3]) -> usize {
    for i in 1..=52 {
        if found[0][i] && found[1][i] && found[2][i] {
            return i;
        }
    }
    0
}