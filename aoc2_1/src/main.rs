use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let f = match File::open("input.txt") {
        Ok(f) => {f},
        Err(_) => {return;},
    };
    let mut reader = BufReader::new(f);
    let mut total_score = 0;

    let char_to_index : HashMap<char, usize> = HashMap::from([
        ('A', 0), ('B', 1), ('C', 2),
        ('X', 0), ('Y', 1), ('Z', 2)
    ]);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(_) => {}
            Err(_) => {break;},
        };
        // Opponent at 0, me at 2
        let opponent= char_to_index[&line.chars().nth(0).unwrap()];
        let me = char_to_index[&line.chars().nth(2).unwrap()];
        total_score += SCORE_MATRIX[opponent][me]// calculate_score(&opponent, &me);

    }
    println!("{:?}", total_score);
}


// SCORE_MATRIX[Opponent, Me]
const SCORE_MATRIX : [[u32;3]; 3] = 
//my rock, pap, sci
    [[1+3, 2+6, 3+0], // opponent rock
     [1+0, 2+3, 3+6], // Paper
     [1+6 ,2+0, 3+3]];// Scissors
