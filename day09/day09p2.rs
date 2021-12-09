use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let nx: usize = txt.lines().next().unwrap().len();
    let ny: usize = txt.split("\n").count();
    let hmap: Vec<i32> = txt.replace("\n","").chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    
    // map of basin IDs
    let mut bmap = vec![-1;nx*ny];

    let mut next_b = 0;
    let mut bsizes: Vec<usize> = Vec::new();
    for i in 0..hmap.len() {
        // 9 or have basin
        if hmap[i] == 9 || bmap[i] != -1 {
            continue;
        }
        
        let x = i % nx;
        let y = i / nx;
        
        if x > 0 && hmap[i] >= hmap[y*nx + x - 1] {
            continue;
        }
        if x < nx-1 && hmap[i] >= hmap[y*nx + x + 1] {
            continue;
        }
        if y > 0 && hmap[i] >= hmap[(y - 1)*nx + x] {
            continue;
        }
        if y < ny-1 && hmap[i] >= hmap[(y + 1)*nx + x] {
            continue;
        }
        
        // low point
        let bsize = flow_up(&hmap, &mut bmap, i, next_b, nx, ny);
        next_b += 1;
        bsizes.push(bsize);
    }
    
    println!("{:?}", bsizes);
    bsizes.sort();
    println!("{:?}", bsizes.iter().rev().take(3).product::<usize>());
}

fn flow_up(hmap: &Vec<i32>, bmap: &mut Vec<i32>, i: usize, b: i32, nx: usize, ny: usize) -> usize {
    // call on low point, then on higher points
    
    // 9 or have basin
    if hmap[i] == 9 || bmap[i] != -1 {
        return 0;
    }
    let mut nmark = 1usize;
    bmap[i] = b;

    let x = i % nx;
    let y = i / nx;

    if x > 0 && hmap[i] < hmap[y*nx + x - 1] {
        nmark += flow_up(hmap, bmap, y*nx + x - 1, b, nx, ny);
    }
    if x < nx-1 && hmap[i] < hmap[y*nx + x + 1] {
        nmark += flow_up(hmap, bmap, y*nx + x + 1, b, nx, ny);
    }
    if y > 0 && hmap[i] < hmap[(y - 1)*nx + x] {
        nmark += flow_up(hmap, bmap, (y - 1)*nx + x, b, nx, ny);
    }
    if y < ny-1 && hmap[i] < hmap[(y + 1)*nx + x] {
        nmark += flow_up(hmap, bmap, (y + 1)*nx + x, b, nx, ny);
    }
    
    nmark
}
