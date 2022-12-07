use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut top_score = 0;
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
            if current_score >= top_score {
                top_score = current_score;
            }
            current_score = 0;
        };
        
        // let r = match line.trim().parse::<u32>() {
        //     Ok(n) => {println!("{}", n)},
        //     Err(_) => {},
        // };
        
    }
    
    println!("{top_score}");
    Ok(())
}