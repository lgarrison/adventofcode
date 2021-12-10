use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();    
    
    let openers = vec!['{', '[', '(', '<'];
    let closers = vec!['}', ']', ')', '>'];
    let scores  = vec![3, 2, 1, 4];
    
    let mut linescores = vec![0u64;0];
    
    'nextline: for line in txt.lines() {
        let mut symstack: Vec<char> = Vec::new();
        for c in line.chars() {
            if openers.contains(&c) {
                symstack.push(c);
            } else {
                let p = symstack.pop().unwrap();
                if openers.iter().position(|&o| o == p) != closers.iter().position(|&o| o == c) {
                    //errscore += scores[closers.iter().position(|&o| o == c).unwrap()];
                    //errlines += 1;
                    continue 'nextline;
                }
            }
        }
        
        // end of line
        //println!("symstack.len() {}", symstack.len());
        let mut score = 0u64;
        while symstack.len() > 0 {
            let opener = symstack.pop().unwrap();
            let opos = openers.iter().position(|&o| o == opener).unwrap();
            let _rightcloser = closers[opos];
            
            score *= 5;
            score += scores[opos];
        }
        //println!("score {}", score);
        linescores.push(score);
    }
    
    linescores.sort_unstable();
    println!("median score {}", linescores[linescores.len()/2]);
    
    //println!("errlines {}", errlines);
}
