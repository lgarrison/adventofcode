use std::fs;
use std::ops::RangeInclusive;
use std::cmp;

#[derive(Debug, Clone)]
struct Cuboid {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
    on: bool,
    overlappers: Vec<Cuboid>,
}

#[derive(Debug)]
struct Engine {
    cubes: Vec<Cuboid>,
    //nset: i64
}

fn overlap_ranges(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> i64 {
    let min = cmp::min(r1.start(), r2.start());
    let max = cmp::max(r1.end(), r2.end());
    let s1 = r1.end() - r1.start() + 1;
    assert!(s1 >= 0);
    
    let s2 = r2.end() - r2.start() + 1;
    assert!(s2 >= 0);
    let mut lap = (s1 + s2) - (max - min + 1);
    lap = cmp:: max(0, lap);
    lap
}

impl Cuboid {
    fn overlap(&self, other: &Cuboid) -> i64 {
        //let sign = if self.on { if other.on { 1 } else { 0 } } else if other.on { 0 } else { -1 };
        let ox = overlap_ranges(&self.x, &other.x);
        let oy = overlap_ranges(&self.y, &other.y);
        let oz = overlap_ranges(&self.z, &other.z);
        
        ox * oy * oz
    }
    
    fn count(&self) -> i64 {
          (self.x.end() - self.x.start() + 1)
        * (self.y.end() - self.y.start() + 1)
        * (self.z.end() - self.z.start() + 1)
    }
    
    fn intersect(&self, other: &Cuboid) -> Cuboid {
        let xmin = *cmp::max(self.x.start(), other.x.start());
        let xmax = *cmp::min(self.x.end(), other.x.end());
        let ymin = *cmp::max(self.y.start(), other.y.start());
        let ymax = *cmp::min(self.y.end(), other.y.end());
        let zmin = *cmp::max(self.z.start(), other.z.start());
        let zmax = *cmp::min(self.z.end(), other.z.end());
        
        assert!(xmax >= xmin);
        assert!(ymax >= ymin);
        assert!(zmax >= zmin);
        
        let on = if self.on {
                     if other.on {
                         false
                     } else {
                         true
                     }
                 } else {
                     false
                 };
        
        Cuboid{x: xmin..=xmax,
               y: ymin..=ymax,
               z: zmin..=zmax,
               on: on,
               overlappers: Vec::new(),
            }
    }
    
    fn add_overlapper(&mut self, lapper: Cuboid) {
        for o in &mut self.overlappers {
            if lapper.overlap(o) != 0 {
                let mut isect = lapper.intersect(o);
                isect.on = !o.on;
                o.add_overlapper(isect);
            }
        }
        self.overlappers.push(lapper);
    }
    
    fn count_recursive(&self) -> i64 {
        let mut nset = if self.on { self.count() } else { -self.count() };
        for o in &self.overlappers {
            nset += o.count_recursive();
        }
        nset
    }
    
    fn print_recursive(&self, depth: usize) {
        println!("{}- {:?} ({:?})", " ".repeat(depth*4), self.on, self.count());
        for o in &self.overlappers {
            o.print_recursive(depth+1);
        }
    }
}

impl Engine {
    fn set(&mut self, new: Cuboid) {
        //self.nset += cuboid.count();
        
        for c in &mut self.cubes {
            if new.overlap(c) != 0 {
                if c.on && new.on {
                    let isect = new.intersect(c);
                    assert!(!isect.on);
                    c.add_overlapper(isect);
                }

                if c.on && !new.on {
                    let isect = new.intersect(c);
                    assert!(!isect.on);
                    c.add_overlapper(isect);
                }
            }
        }
        if new.on {
            self.cubes.push(new);
        }
    }
    
    fn count(&self) -> i64 {
        let mut nset = 0i64;
        for c in &self.cubes {
            assert!(c.on);
            nset += c.count_recursive();
        }
        nset
    }
}

fn parse_cubes(txt: &str) -> Vec<Cuboid> {
    let mut cubes:  Vec<Cuboid> = Vec::new();
    for line in txt.lines() {
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
            assert!(xlow <= xhigh);
                        
            ranges.push(xlow..=xhigh);
        }
        let cube = Cuboid{x: ranges.remove(0),
                          y: ranges.remove(0),
                          z: ranges.remove(0),
                          on: on,
                          overlappers: Vec::new(),
                    };
        //println!("{:?}", cube);
        
        cubes.push(cube);
    }
    
    cubes
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let cubes = parse_cubes(&txt);
    
    let mut engine = Engine{cubes: Vec::new(),
                        //nset: 0
                    };
    
    
    for cube in cubes {
        engine.set(cube.clone());
    }
    
    for cube in &engine.cubes {
        //cube.print_recursive(0);
    }
    
    println!("{}", engine.count());
}