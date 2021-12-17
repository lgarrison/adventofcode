use std::fs;
use regex::Regex;
use std::ops::RangeInclusive;
use std::cmp;

fn parse(txt: &str) -> Vec<RangeInclusive<i32>> {
    let re = Regex::new(r".*x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let caps = re.captures(txt).unwrap();
    
    let mut p: Vec<i32> = Vec::new();
    for i in 1..=4 {
        p.push(caps.get(i).unwrap().as_str().parse::<i32>().unwrap());
    }
    
    vec![p[0]..=p[1],p[2]..=p[3]]
}

fn step(xy: &mut Vec<i32>, vxy: &mut Vec<i32>) {
    xy[0] += vxy[0];
    xy[1] += vxy[1];
    vxy[0] -= 1*vxy[0].signum();
    vxy[1] -= 1;
}

fn intarget(xy: &Vec<i32>, target: &Vec<RangeInclusive<i32>>) -> bool {
    target[0].contains(&xy[0]) && target[1].contains(&xy[1])
}

fn sim(mut vxy: &mut Vec<i32>, target: &Vec<RangeInclusive<i32>>) -> (bool, Vec<i32>, i32) {
    let mut xy = vec![0i32,0i32];
    let mut ymax = i32::MIN;
    
    loop {
        step(&mut xy, &mut vxy);
        ymax = cmp::max(xy[1], ymax);
        //println!("{:?} {:?}", xy, vxy);
        if intarget(&xy, &target){
            break;
        }
        
        if vxy[0] == 0 && !target[0].contains(&xy[0]) {
            // no x vel
            break;
        }
        
        if xy[1] < *target[1].start() {  // start always < end
            // below the region
            break;
        }
    }
    
    (intarget(&xy, &target), xy, ymax)
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let target = parse(&txt);
    
    let mut goal = false;
    let mut xy = vec![0,0];
    let mut ymax = i32::MIN;
    let mut ngoal = 0;
    
    for vx in 0..=*target[0].end() {
        for vy in -1000..10000 {
            let mut vxy = vec![vx,vy];
            let res = sim(&mut vxy, &target);
            goal = res.0;
            xy = res.1;
            if goal {
                //println!("{:?},{:?}", vx, vy);
                ngoal += 1;
                ymax = cmp::max(ymax,res.2);
            }
        }
    }
    
    println!("ymax {:?}, ngoal {:?}", ymax, ngoal);
    //println!("{:?} {:?}", goal, xy);
}
