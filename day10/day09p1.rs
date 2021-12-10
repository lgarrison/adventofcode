use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();    
    
    let openers = vec!['{', '[', '(', '<'];
    let closers = vec!['}', ']', ')', '>'];
    let scores  = vec![1197, 57, 3, 25137];
    
    let mut errscore = 0;
    let mut errlines = 0;
    'line: for line in txt.lines() {
        let mut symstack: Vec<char> = Vec::new();
        for c in line.chars() {
            if openers.contains(&c) {
                symstack.push(c);
            } else {
                let p = symstack.pop().unwrap();
                if openers.iter().position(|&o| o == p) != closers.iter().position(|&o| o == c) {
                    errscore += scores[closers.iter().position(|&o| o == c).unwrap()];
                    errlines += 1;
                    continue 'line;
                }
            }
        }
    }
    
    println!("errscore {}", errscore);
    println!("errlines {}", errlines);
}
