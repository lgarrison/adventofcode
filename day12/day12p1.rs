use std::fs;
use std::collections::HashMap;

fn issmall(c: &str) -> bool {
    c.chars().all(|x| x.is_ascii_lowercase())
}

fn visit_neighbors(edges: &HashMap<String,Vec<String>>, node: &str, visited: &mut Vec<String>, doubled: bool) -> i32 {
    if node == "end" {
        return 1i32;
    }
    
    let mut npath = 0;
    if issmall(node){
        visited.push(node.to_string());
    }
    
    for neigh in &edges[node] {
        let count = visited.iter().filter(|v| *v == neigh).count();
        if neigh == "start" {
            continue;
        }
        if count == 0 {
           npath += visit_neighbors(edges, neigh, visited, doubled);
        }
        else if count == 1 && !doubled {
           npath += visit_neighbors(edges, neigh, visited, true);
        }
    }
    
    if issmall(node){
        visited.pop();
    }
    
    npath
}

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();    
    
    let mut edges: HashMap<String,Vec<String>> = HashMap::new();
    
    for line in txt.lines() {
        let split: Vec<&str> = line.split("-").collect();
        let n1 = split[0];
        let n2 = split[1];
        if !edges.contains_key(n1) {
            edges.insert(n1.to_string(), Vec::new());
        }
        if !edges.contains_key(n2) {
            edges.insert(n2.to_string(), Vec::new());
        }
        edges.get_mut(n1).unwrap().push(n2.to_string());
        edges.get_mut(n2).unwrap().push(n1.to_string());
    }
    
    let npath = visit_neighbors(&edges, "start", &mut Vec::new(), false);
    
    //println!("{:?}", edges);
    println!("{:?}", npath);
}
