use std::fs;

const N: usize = 2000;

fn dofold(grid: &mut Vec<i8>, max: &mut (usize,usize), fold: isize) {
    if fold < 0 {
        let ufold: usize = (-fold) as usize;
        
        for i in 0..max.0 {
            for j in ufold..max.1 {
                if j > 2*ufold {
                    break;
                }
                let refl = 2*ufold - j;
                if grid[i*N + j] > 0 {
                    grid[i*N + refl] = grid[i*N + j];
                }
            }
        }
        
        max.1 = ufold;
    } else {
        let ufold: usize = fold as usize;
        
        for i in ufold..max.0 {
            for j in 0..max.1 {
                if i > 2*ufold {
                    break;
                }
                let refl = 2*ufold - i;
                if grid[i*N + j] > 0 {
                    grid[refl*N + j] = grid[i*N + j];
                }
            }
        }
        
        max.0 = ufold;
    }
}

fn sumdots(grid: &Vec<i8>, max: &(usize, usize)) -> i32 {
    let mut sum = 0i32;
    for i in 0..max.0 {
        for j in 0..max.1 {
            sum += grid[i*N + j] as i32;
        }
    }
    
    sum
}

fn printdots(grid: &Vec<i8>, max: &(usize, usize)) {
    for j in 0..max.1 {
        for i in 0..max.0 {
            print!("{}", if grid[i*N + j] > 0 {"#"} else {"."} );
        }
        print!("\n");
    }
}

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();    
    
    let mut grid: Vec<i8> = vec![0i8;N*N];
    let mut folds: Vec<isize> = Vec::new();
    let mut max = (0,0);
    
    for line in txt.lines() {
        if line.len() == 0 {
            continue;
        }
        
        if line.starts_with("fold") {
            let s = line.split(" ").last().unwrap();
            let mut eqn = s.split("=");
            let xy = eqn.next().unwrap();
            let val = eqn.last().unwrap().parse::<isize>().unwrap();
            
            if xy == "x"{
                folds.push(val);
            } else {
                folds.push(-val);
            }
            
            continue;
        }
        
        let s: Vec<usize> = line.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let x = s[0];
        let y = s[1];
        assert!(x < N && y < N);
        max.0 = std::cmp::max(x+1, max.0);
        max.1 = std::cmp::max(y+1, max.1);
        grid[x*N + y] = 1;
    }
    
    println!("{:?}", max);
    
    for f in folds {
        dofold(&mut grid, &mut max, f);
        //break;
    }
    
    println!("dots: {:?}", sumdots(&grid, &max));
    
    printdots(&grid, &max);
}
