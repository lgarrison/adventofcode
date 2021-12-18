use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let path = "test.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut nums: Vec<Vec<i32>> = Vec::new();
    let mut depths: Vec<Vec<i32>> = Vec::new();
    let mut pranges: Vec<Vec<RangeInclusive<usize>>> = Vec::new();
    
    for line in txt.lines() {
        let mut linenums = Vec::new();
        let mut linedepths = Vec::new();
        let mut linepstarts = Vec::new();
        let mut linepranges = Vec::new();
        
        let mut i = 0usize;
        let mut depth = 0;
        
        while i < line.len() {
            match &line[i..i+1] {
                "[" => {
                    i += 1;
                    depth += 1;
                    linepstarts.push(linenums.len());
                    continue;
                },
                "]" => {
                    i += 1;
                    depth -= 1;
                    assert!(depth >= 0);
                    let prange = RangeInclusive::new(linepstarts.pop().unwrap(),linenums.len());
                    linepranges.push(prange);
                    continue;
                },
                "," => {
                    i += 1;
                    continue;
                }
                _ => { }
            }
            
            let numstr = *line[i..].split(|s| [',',']','['].contains(&s)).collect::<Vec<&str>>().first().unwrap();
            i += numstr.len();
            let num = numstr.parse::<i32>().unwrap();
            
            linenums.push(num);
            linedepths.push(depth);
        }
        assert!(depth == 0);
        assert!(linepstarts.len() == 0);
        
        nums.push(linenums);
        depths.push(linedepths);
        pranges.push(linepranges);
    }
    
    println!("{:?}\n\n", nums);
    println!("{:?}\n\n", depths);
    println!("{:?}\n\n", pranges);
}
