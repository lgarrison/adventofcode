use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    
    let mut gamma = 0;
    let mut epsilon = 0;
    
    let mut high: [i32; 12] = [0; 12];
    let mut nlines = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(numstr) = line {
                nlines += 1;
                for (i,char) in numstr.chars().enumerate() {
                    if char == '1' {
                        high[i] += 1;
                    }
                }
            }
        }
    }
    
    for i in 0..12 {
        if high[i] > nlines/2 {
            gamma += 1 << (11-i);
        } else {
            epsilon += 1 << (11-i);
        }
    }
    
    println!("lines: {}", nlines);
    println!("{:?}", high);
    
    println!("gamma: {}",gamma);
    println!("epsilon: {}",epsilon);
    println!("product: {}",gamma*epsilon);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
