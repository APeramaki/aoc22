use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let f = match File::open("aoc22_3_1_input.txt") {
        Ok(f) => {f},
        Err(_) => {return;},
    };
    let mut reader = BufReader::new(f);
    let mut total_score: usize = 0;
    
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(_) => {}
            Err(_) => {break;},
        };

        let line_len = line.len();
        let first_half  = &line[..line_len/2  ];
        let second_half = &line[  line_len/2..];
        let mut found_in_first_half = [false; 53];

        for c in first_half.chars() {
            found_in_first_half[char_to_priority(c)] = true;
        }

        for c in second_half.chars() {
            let char_prio = char_to_priority(c);
            if found_in_first_half[char_prio] {
                total_score += char_prio;
                break;
            }
        }

    }
    println!("{:?}", total_score);
}

fn char_to_priority(c : char) -> usize {
    if c.is_lowercase() {
        return (c.to_digit(36).unwrap() - 9) as usize;
    }
    (c.to_digit(36).unwrap() - 9 + 26) as usize
}