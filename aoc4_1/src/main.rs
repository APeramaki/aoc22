use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let f = match File::open("aoc22_4_1_input.txt") {
        Ok(f) => {f},
        Err(_) => {return;},
    };
    let mut reader = BufReader::new(f);
    let mut total_score: usize = 0;
    
    loop {        
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(_) => {line = line.trim().to_string()}
            Err(_) => {break;},
        };
        let areas = parse_areas(&line);
        total_score += if includes(areas) { 1 } else { 0 };
    }
    println!("{:?}", total_score);
}

fn parse_areas(line :&str) -> [[usize; 2]; 2]{
    let mut split = line.split(',');
    let mut first_elf = split.next().unwrap().split('-');
    let mut second_elf = split.next().unwrap().split('-');
    [
        [first_elf.next().unwrap().parse().unwrap(), first_elf.next().unwrap().parse().unwrap()],
        [second_elf.next().unwrap().parse().unwrap(), second_elf.next().unwrap().parse().unwrap()]
    ]
}

fn includes (areas : [[usize; 2]; 2]) -> bool {
    if areas[0][0] >= areas[1][0] && areas[0][0] <= areas[1][1] && areas[0][1] <= areas[1][1]{
        return true; // First is inside second
    }
    if areas[1][0] >= areas[0][0] && areas[1][0] <= areas[0][1] && areas[1][1] <= areas[0][1]{
        return true; // Second is inside first
    }
    
    false
}