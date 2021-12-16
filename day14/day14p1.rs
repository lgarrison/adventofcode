use std::fs;
use std::collections::HashMap;

fn polymerize(chain: &Vec<char>, rules: &HashMap<&str,char>) -> Vec<char> {
    let mut newchain: Vec<char> = Vec::new();
    
    for i in 0..chain.len()-1 {
        newchain.push(chain[i]);
        if let Some(ins) = rules.get(&chain[i..i+2].iter().collect::<String>().as_str()) {
            newchain.push(*ins);
        }
    }
    
    newchain.push(*chain.last().unwrap());
    
    newchain
}

fn doscore(chain: &Vec<char>) -> i32 {
    let mut counts: HashMap<char,i32> = HashMap::new();
    
    for &c in chain {
        (*(counts.entry(c).or_insert(0))) += 1
    }
    
    let vmax = counts.values().max().unwrap();
    let vmin = counts.values().min().unwrap();
    
    return vmax - vmin;
}

fn main () {
    let path = "exp1.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut lines = txt.lines();
    let mut chain: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    
    let mut rules = HashMap::new();
    for line in lines {
        let s = line.split(" -> ").collect::<Vec<_>>();
        rules.insert(s[0], s[1].chars().next().unwrap());
    }
    
    for i in 0..10 {
        chain = polymerize(&chain, &rules);
        println!("i: {} -- chain.len(): {}", i, chain.len());
    }
    
    let score = doscore(&chain);
    
    //println!("chain {:?}", chain);
    println!("score {:?}", score);
}
