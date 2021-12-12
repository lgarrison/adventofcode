use std::fs;

const NGRID: usize = 10;

fn try_inc(grid: &mut [[i32;NGRID];NGRID], i: usize, j: usize, ioff: isize, joff: isize) {
    let ii: isize = (i as isize) + ioff;
    let jj: isize = (j as isize) + joff;
    
    if ii >= 0 && ii < (NGRID as isize) && jj >= 0 && jj < (NGRID as isize)  {
        if grid[ii as usize][jj as usize] != -1 {  // flashed
            grid[ii as usize][jj as usize] += 1;
        }
    }
}

fn flash(grid: &mut [[i32;NGRID];NGRID]) -> usize {
    let mut nflash = 0;
    
    // flash! ah-ahhh
    for i in 0..NGRID {
        for j in 0..NGRID {
            if grid[i][j] > 9 {
                grid[i][j] = -1;
                nflash += 1;
                
                try_inc(grid, i, j, -1, 0);
                try_inc(grid, i, j,  1, 0);
                try_inc(grid, i, j,  0,-1);
                try_inc(grid, i, j,  0, 1);
                
                try_inc(grid, i, j, -1, -1);
                try_inc(grid, i, j, -1,  1);
                try_inc(grid, i, j,  1, -1);
                try_inc(grid, i, j,  1,  1);
            }
        }
    }
    
    nflash
}

fn reset_flashers(grid: &mut [[i32;NGRID];NGRID]) {
    for i in 0..NGRID {
        for j in 0..NGRID {
            if grid[i][j] == -1 {
                grid[i][j] = 0;
            }            
        }
    }
}

fn step(grid: &mut [[i32;NGRID];NGRID]) -> usize {
    for i in 0..NGRID {
        for j in 0..NGRID {
            grid[i][j] += 1;
        }
    }
    
    let mut totflash = 0usize;
    while let nflash = flash(grid) {
        if nflash == 0 {
            break;
        }
        totflash += nflash;
    }
    
    reset_flashers(grid);
    
    totflash
}

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();    
    
    let mut grid = [[-1i32;NGRID];NGRID];
    
    for (i,line) in txt.lines().enumerate() {
        for (j,c) in line.chars().enumerate() {
            grid[i][j] = c.to_digit(10).unwrap() as i32;
        }
    }
    
    let mut totflash = 0;
    for _ in 0..100 {
        totflash += step(&mut grid);
    }
    
    println!("total flashes: {:?}", totflash);
}
