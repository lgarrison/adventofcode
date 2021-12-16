use std::fs;
use std::collections::HashMap;

fn polymerize(chain: &HashMap<String,i64>, rules: &HashMap<String,(String,String)>) -> HashMap<String,i64> {
    let mut newchain: HashMap<String,i64> = HashMap::new();
    
    for (k,v) in chain {
        let n = &rules[k];
        *newchain.entry((n.0).clone()).or_insert(0) += v;
        *newchain.entry((n.1).clone()).or_insert(0) += v;
    }
    
    newchain
}

fn doscore(chain: &HashMap<String,i64>, f: &char, l: &char) -> i64 {
    let mut counter: HashMap<char,i64> = HashMap::new();
    
    for (k,v) in chain {
        for c in k.chars() {
            *counter.entry(c).or_insert(0) += v;
        }
    }
    
    *counter.get_mut(f).unwrap() -= 1;
    *counter.get_mut(l).unwrap() -= 1;
    
    for (_, val) in counter.iter_mut() {
        *val /= 2;
    }
    
    *counter.get_mut(f).unwrap() += 1;
    *counter.get_mut(l).unwrap() += 1;
    
    let vmax = counter.values().max().unwrap();
    let vmin = counter.values().min().unwrap();
    
    vmax - vmin
}

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut lines = txt.lines();
    let strchain = lines.next().unwrap();
    lines.next();
    
    // str chain to map of pair counts
    let mut chain: HashMap<String,i64> = HashMap::new();
    for i in 0..strchain.len()-1 {
        let pair = strchain[i..i+2].to_string();
        *chain.entry(pair).or_insert(0) += 1;
    }
    
    let mut rules: HashMap<String,(String,String)> = HashMap::new();
    for line in lines {
        let s = line.split(" -> ").collect::<Vec<_>>();
        let k = s[0].to_string();
        let v1 = k[..1].to_string() + s[1];
        let v2 = s[1].to_string() + &k[1..2];
        
        rules.insert(k, (v1,v2));
    }
  
    //println!("{:?}", chain);
    for _i in 0..40 {
        chain = polymerize(&chain, &rules);
    }
    //println!("{:?}", chain);
    
    let score = doscore(&chain, &strchain.chars().next().unwrap(), &strchain.chars().last().unwrap());
    
    //println!("chain {:?}", chain);
    println!("score {:?}", score);
}
