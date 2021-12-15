use std::fs;

fn main () {
    let path = "input.txt";
    let ninit = 100;
    let n = 5*ninit;
    let txt = fs::read_to_string(path).unwrap();    
    
    let mut _risk: Vec<i32> = Vec::new();
    
    for line in txt.lines() {
        for c in line.chars() {
            _risk.push(c.to_digit(10).unwrap() as i32);
        }
    }
    
    let mut risk: Vec<i32> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    
    for i in 0..n*n {
        let mut newr = _risk[ ninit*((i/n)%ninit) + (i%n)%ninit ];
        newr += ((i%n)/ninit + (i/n)/ninit) as i32;
        newr = (newr-1) % 9 + 1;
        risk.push(newr);
    
        dist.push(i32::MAX);
        visited.push(false);
    }
    
    let mut cur = 0usize;
    dist[cur] = 0;
    loop {
        if cur%n >= 1 && !visited[cur-1] {
            //println!("{} left", cur);
            let neigh = cur-1;
            let newdist = dist[cur] + risk[neigh];
            // left
            if newdist < dist[neigh] {
                dist[neigh] = newdist;
            }
        }
        if cur%n < n-1 && !visited[cur+1] {
            //println!("{} right" ,cur);
            // right
            let neigh = cur+1;
            let newdist = dist[cur] + risk[neigh];
            if newdist < dist[neigh] {
                dist[neigh] = newdist;
            }
        }
        if cur/n >= 1 && !visited[cur-n] {
            //println!("{} up", cur);
            // up
            let neigh = cur-n;
            let newdist = dist[cur] + risk[neigh];
            if newdist < dist[neigh] {
                dist[neigh] = newdist;
            }
        }
        if cur/n < n-1 && !visited[cur+n] {
            //println!("{} down", cur);
            // down
            let neigh = cur+n;
            let newdist = dist[cur] + risk[neigh];
            if newdist < dist[neigh] {
                dist[neigh] = newdist;
            }
        }
        visited[cur] = true;
        
        let mut next = 0;
        let mut nextcost = i32::MAX;
        for i in 0..risk.len() {
            if !visited[i] && dist[i] < nextcost {
                nextcost = dist[i];
                next = i;
            }
        }
        cur = next;
        
        if cur == risk.len()-1 {
            break;
        }
    }
    
    println!("{:?}", dist[n*n-1]);
}
