use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    
    let mut fwd = 0;
    let mut depth = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(cmd) = line {
                let mut iter = cmd.split_whitespace();
                let cmd = iter.next().unwrap();
                let amt: i32 = iter.next().unwrap().parse().unwrap();
                
                match cmd {
                    "forward" => fwd += amt,
                    "down" => depth += amt,
                    "up" => depth -= amt,
                    _ => println!("error!"),
                }
            }
        }
    }
    
    println!("forward: {}",fwd);
    println!("depth: {}",depth);
    println!("product: {}",fwd*depth);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
