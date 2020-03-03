use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut buffer = Vec::<i64>::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(change) = line {
                buffer.push(change.trim().parse::<i64>().unwrap());
            }
        }

        let mut frequency: i64 = 0;
        let mut frequency_memo = HashSet::<i64>::new();
        let answer: i64;
        'outer: loop {
            for change in &buffer {
                frequency += change;
                if frequency_memo.contains(&frequency) {
                    answer = frequency;
                    break 'outer;
                } else {
                    frequency_memo.insert(frequency);
                }
            }
        }
        println!("{}", answer);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}