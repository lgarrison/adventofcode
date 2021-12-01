use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        
        let mut depths = Vec::new();
        
        for line in lines {
            if let Ok(depthstr) = line {
                let depth = depthstr.parse::<i32>().unwrap();
                depths.push(depth)
            }
        }
        
        let wdepths = depths.windows(3);
        let mut last_sum3 = -1;
        let mut numinc = 0;
        for s in wdepths {
            let sum3: i32 = s.iter().sum();
            //println!("{:?}",sum3);
            
            if last_sum3 < 0 {
                last_sum3 = sum3;
                continue;
            }
            
            if sum3 > last_sum3 {
                numinc += 1;
            }
            last_sum3 = sum3;
        }
        println!("{:?}", numinc);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
