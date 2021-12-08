use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut nmatch1478 = 0;
    for line in txt.lines() {
        let input: Vec<&str> = line.split(" ").collect();
        let ibar = input.iter().position(|v| *v == "|").unwrap();
        let patterns = &input[..ibar];
        let outputs = &input[ibar+1..];
        
        let mut digpat = ["";10];
        for o in outputs {
            nmatch1478 += [2,4,3,7].contains(&(o.len() as i32)) as i32;
        }
        
        /*for p in patterns {
            match p.len() {
                2 => digpat[1] = p,
                4 => digpat[4] = p,
                3 => digpat[7] = p,
                7 => digpat[8] = p,
                _ => (),  //println!("len {} ambiguous...",p.len()),
            }
        }
        
        println!("{:?}", digpat);
        println!("{:?}", outputs);
        nmatch1478 += outputs.iter().filter(|&o| digpat.contains(o)).count();*/
    }
    
    println!("nmatch1478 {}", nmatch1478);
}
