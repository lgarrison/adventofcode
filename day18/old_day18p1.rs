use std::fs;

#[derive(Debug)]
enum Element
{
    Pair(Box<(Element,Element)>),
    Int(i32),
}


fn main() {
    let path = "test.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let nums: Vec<Element> = Vec::new();
    
    for line in txt.lines() {
        let mut num = Element::Pair(Box::new((Element::Int(0),Element::Int(0))));
        let mut i = 0usize;
        let mut depth = 0;
        
        loop {
            if &line[i..i+1] == "[" {
                println!("open");
                let p = Element::Pair(Box::new((Element::Int(0),Element::Int(0))));
                match num {
                    Element::Pair(ref mut q) => {q.0 = p;},
                    Element::Int(_) => {},
                }
                i += 1;
                depth += 1;
                continue;
            }
            if &line[i..i+1] == "]" {
                println!("close");
                i += 1;
                depth -= 1;
                assert!(depth >= 0);
                continue;
            }
            
            if let Ok(num) = line.split(",").first().parse::<i32>() {
                
            }
        }
        
        nums.push(num);
    }
}
