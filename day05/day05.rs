use std::fs;
use std::cmp;

const BSZ: i32 = 1000;

fn parse_starts_ends(txt: &str) -> Vec<[i32;4]> {
    let lines = txt.split("\n");
    
    let mut ends: Vec<[i32;4]> = Vec::new();
    for line in lines {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let xy1: Vec<i32> = coords[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let xy2: Vec<i32> = coords[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        ends.push([xy1[0], xy1[1], xy2[0], xy2[1]]);
    }
    
    ends
}

fn is_rect(p: &[i32;4]) -> bool {
    (p[0] == p[2]) != (p[1] == p[3])
}

fn mark(ends: &Vec<[i32;4]>) -> Vec<i32> {
    let mut board = vec![0; (BSZ*BSZ) as usize];
    
    for p in ends {
        let x1 = p[0];
        let x2 = p[2];
        let dx = x2 - x1;
        let y1 = p[1];
        let y2 = p[3];
        let dy = y2 - y1;
        
        if dx != 0 {
            assert!(dy == 0 || dy.abs() == dx.abs());
        }
        if dy != 0 {
            assert!(dx == 0 || dx.abs() == dy.abs());
        }
        
        let step = dx.signum()*BSZ + dy.signum();
        let imin = cmp::min(BSZ*x1 + y1, BSZ*x2 + y2) as usize;
        let imax = cmp::max(BSZ*x1 + y1, BSZ*x2 + y2) as usize;
        //println!("step {} imin {} imax {}", step, imin, imax);
    
        for i in (imin..=imax).step_by(step.abs() as usize) {
            board[i] += 1;
        }
    }
    
    board
}

fn count(board: &Vec<i32>) -> i32 {
    board.iter().map(|&v| (v >= 2) as i32).sum::<i32>()
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut ends = parse_starts_ends(&txt);
    
    //ends.retain(is_rect);
    
    let board = mark(&ends);
    
    println!("{:?}", ends);
    for i in 0..BSZ {
        println!("{:?}", &board[(i*BSZ) as usize..((i+1)*BSZ) as usize]);
    }
    
    println!("count: {}", count(&board));
}
