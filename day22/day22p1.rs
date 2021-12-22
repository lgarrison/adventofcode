use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Cuboid {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
    on: bool ,
}

#[derive(Debug)]
struct Engine {
    state: Vec<bool>,
    n1d: i64
}

impl Engine {
    fn set(&mut self, cuboid: &Cuboid) {
        let nhalf = self.n1d/2;
        for i in cuboid.x.clone() {
            if !(-nhalf..=nhalf).contains(&i) {
                continue;
            }
            for j in cuboid.y.clone() {
                if !(-nhalf..=nhalf).contains(&j) {
                    continue;
                }
                for k in cuboid.z.clone() {
                    if !(-nhalf..=nhalf).contains(&k) {
                        continue;
                    }
                    let idx: usize = ((i + nhalf)*self.n1d*self.n1d
                        + (j + nhalf)*self.n1d
                        + k + nhalf) as usize;
                    self.state[idx] = cuboid.on;
                }
            }
        }
    }
    
    fn count(&self) -> i64 {
        self.state.iter().map(|&v| v as i64).sum()
    }
}

fn parse_cubes(txt: &str, lim: i64) -> Vec<Cuboid> {
    let mut cubes:  Vec<Cuboid> = Vec::new();
    'nextline: for line in txt.lines() {
        let mut s = line.split(" ");
        let on = s.next().unwrap() == "on";
        let mut ss = s.next().unwrap().split(",");
        
        let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
        for _ in 0..3 {
            let mut xb = ss.next()
                    .unwrap()
                    .split("=")
                    .last()
                    .unwrap()
                    .split("..")
                    .map(|s| s.parse::<i64>().unwrap());
            let xlow = xb.next().unwrap();
            let xhigh = xb.next().unwrap();
            
            if (xlow < -lim && xhigh < -lim)
                || (xlow > lim && xhigh > lim) {
                continue 'nextline;
            }
            
            ranges.push(xlow..=xhigh);
        }
        let cube = Cuboid{x: ranges.remove(0),
                          y: ranges.remove(0),
                          z: ranges.remove(0),
                          on: on,
                    };
        println!("{:?}", cube);
        
        cubes.push(cube);
    }
    
    cubes
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let cubes = parse_cubes(&txt, 50);
    
    let n1d = 101;
    let mut engine = Engine{state: vec![false;n1d*n1d*n1d],
                        n1d: n1d as i64,
                    };
    
    for cube in cubes {
        engine.set(&cube);
    }
    
    println!("{}", engine.count());
}