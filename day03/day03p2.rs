use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAXWIDTH: i32 = 12;

fn count_highs(vals: &[i32], width: i32) -> Vec<i32> {
    let mut high: Vec<i32> = Vec::new();
    for _ in 0..width {
        high.push(0);
    }
    
    for &v in vals {
        for (i,h) in high.iter_mut().enumerate() {
            *h += ((v & (1 << i)) != 0) as i32;
        }
    }
    
    return high;
}

fn main() {
    let mut vals: Vec<i32> = Vec::new();
    let mut nlines = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(numstr) = line {
                nlines += 1;
                vals.push(i32::from_str_radix(&numstr, 2).unwrap());
            }
        }
    }
    
    for w in (0..MAXWIDTH).rev() {
        let high = count_highs(&vals, MAXWIDTH);
        println!("high {:?}", high);
        let idx = w as usize;
        let nleft = vals.len() as i32;
        if high[idx]*2 >= nleft {
            vals.retain(|&v|v & (1 << w) == 0);
        } else {
            vals.retain(|&v|v & (1 << w) != 0);
        }
        if vals.len() == 1 {
            break;
        }
    }
    println!("vals: {:?}", vals);
    
    println!("lines: {}", nlines);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
