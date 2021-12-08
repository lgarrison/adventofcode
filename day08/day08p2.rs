use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut onums = Vec::new();
    for line in txt.lines() {
        let input: Vec<&str> = line.split(" ").collect();
        let ibar = input.iter().position(|v| *v == "|").unwrap();
        let patterns = &input[..ibar];
        let mut outputs: Vec<String> = (&input[ibar+1..]).iter().map(|s| String::from(*s)).collect();
        
        let mut _outsort: Vec<Vec<char>> = outputs.iter().map(|o| o.chars().collect()).collect::<Vec<Vec<char>>>();
        for _o in &mut _outsort {
            _o.sort();
        }
        for i in 0..outputs.len() {
            outputs[i] = _outsort[i].iter().collect();
        }
        
        //println!("outputs: {:?}",outputs);
        
        let mut digpat = ["";10];
        
        for p in patterns {
            let dig = match p.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => 10,  //println!("len {} ambiguous...",p.len()),
            };
            if dig == 10 {
                continue;
            }
            assert!(digpat[dig] == "");
            digpat[dig] = p;
        }
        
        // top line
        let A: char = digpat[7].chars().filter(|c| !digpat[1].contains(*c)).next().unwrap();
        
        let mut C: char = '0';
        for p in patterns {
            if p.len() == 6 {
                let fournine: Vec<char> = digpat[4].chars().filter(|c| !p.contains(*c)).collect();
                if fournine.len() == 0 {
                    digpat[9] = p;
                    continue;
                }
                
                let _C: Vec<char> = digpat[1].chars().filter(|c| !p.contains(*c)).collect();
                if _C.len() == 1 {
                    C = _C[0];
                    digpat[6] = p;
                } else {
                    assert!(_C.len() == 0);
                    digpat[0] = p;
                }
            }
        }
        assert!(C != '0');
        let F: char = digpat[1].chars().filter(|c| *c != C).next().unwrap();
        
        let D: char = digpat[8].chars().filter(|c| !digpat[0].contains(*c)).next().unwrap();
        
        let B: char = digpat[4].chars().filter(|c| ![C, D, F].contains(c)).next().unwrap();
        
        for p in patterns {
            if p.len() == 5 {
                if p.chars().filter(|c| *c == C).collect::<Vec<char>>().len() == 0 {
                    assert!(digpat[5] == "");
                    digpat[5] = p;
                    
                }
            }
        }
        let G: char = digpat[5].chars().filter(|c| ![A, B, D, F].contains(c)).next().unwrap();
        let E: char = digpat[0].chars().filter(|c| ![A, B, C, F, G].contains(c)).next().unwrap();
        
        //println!("{:?}", digpat);
        for p in patterns {
            if p.len() == 5 {
                println!("len 5: {}", p);
                if p.chars().filter(|c| ![A, C, D, E, G].contains(c)).collect::<Vec<char>>().len() == 0 {
                    assert!(digpat[2] == "");
                    digpat[2] = p;
                }
                if p.chars().filter(|c| ![A, C, D, F, G].contains(c)).collect::<Vec<char>>().len() == 0 {
                    assert!(digpat[3] == "");
                    digpat[3] = p;
                }
            }
        }
        
        // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        // 
        
        println!("A-{} B-{} C-{} D-{} E-{} F-{} G-{}", A, B, C, D, E, F, G);
        println!("{:?}", digpat);
        //println!("{:?}", outputs);
        //nmatch1478 += outputs.iter().filter(|&o| digpat.contains(o)).count();
        
        let mut _digpatsort = digpat.iter().map(|d| d.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        for dps in &mut _digpatsort {
            dps.sort();
        }
        let digpatsort = _digpatsort.iter().map(|d| d.into_iter().collect::<String>()).collect::<Vec<String>>();
        
        let mut onum = 0i32;
        for (i,o) in outputs.iter().rev().enumerate() {
            onum += 10i32.pow(i as u32)*digpatsort.iter().position(|d| d == o).unwrap() as i32;
        }
        onums.push(onum);
    }
    
    println!("onums {:?}", onums);
    println!("onums.sum() = {:}", onums.iter().sum::<i32>());
}
