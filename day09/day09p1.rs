use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let nx: usize = txt.lines().next().unwrap().len();
    let ny: usize = txt.split("\n").count();
    let hmap: Vec<i32> = txt.replace("\n","").chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    
    println!("{} {}", nx, ny);

    let mut risksum = 0;
    for i in 0..hmap.len() {
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
        risksum += hmap[i] + 1;
    }
    
    println!("{}", risksum);
    
}
