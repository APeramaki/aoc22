use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = match File::open("input.txt") {
        Ok(f) => {f},
        Err(_) => {return;},
    };
    let mut reader = BufReader::new(f);
    
    let mut piles: [Vec<char>; 9] = [
            Vec::new(), Vec::new(), Vec::new(),
            Vec::new(), Vec::new(), Vec::new(),
            Vec::new(), Vec::new(), Vec::new(),
        ];
    loop {        
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(1) => {
                // Blank!
                break;
            },
            Ok(_) => {},
            Err(_) => {break;},
        };

        for (i, c) in  line.chars().enumerate() {
            if i % 4 == 1 && c.is_alphabetic() {
                piles[i/4].splice(..0, [c]);
            }
        }
    }

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(1) => {
                println!("Blank!");
                break;
            },
            Ok(_) => {
                line = line.trim_end().to_string();
            },
            Err(_) => {break;},
        };
        
        let mut split = line.split(' ');
        split.next();
        let amount: usize = split.next().unwrap().parse().unwrap();
        split.next();
        let from_pile: usize = split.next().unwrap().parse().unwrap();
        split.next();
        let to_pile: usize = split.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            let from = piles[from_pile - 1].pop().unwrap();
            piles[to_pile - 1].push(from);
        }
    }
    
    for i in 0..9 {
        println!("{:?}", piles[i]);
    }
}
