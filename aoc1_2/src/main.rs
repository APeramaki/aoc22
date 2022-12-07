use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut top_scores = [0; 3];
    let mut current_score = 0;
    loop {
        let mut line = String::new();
        
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(_) => {},
            Err(_) => {break;},
        };

        if let Ok(n) = line.trim().parse::<u32>() {
            current_score += n;
        }else{    
            if current_score >= top_scores[2] {
                top_scores = reorder(current_score, top_scores);
            }
            current_score = 0;
        };    
    }

    let sum = top_scores[0] + top_scores[1] + top_scores[2];
    println!("{:?}",sum);
    Ok(())
}

fn reorder(current_score : u32, old : [u32; 3])-> [u32; 3] {
    let mut new = [0, 0, 0];
    for (i, &item ) in old.iter().enumerate() {
        if current_score >= old[i] {
            for j in i..=1 {
                new[j + 1] = old[j];
            }
            new[i] = current_score;
            break;
        } else {
            new[i] = item;
        }
    }
    new
}