use std::fs;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(i32, i32, i32, bool);

impl Point {
    fn dist2(&self) -> i64 {
        self.0 as i64 * self.0 as i64 +
        self.1 as i64 * self.1 as i64 +
        self.2 as i64 * self.2 as i64
    }
    
    fn index(&self, other: i32) -> Option<usize> {
        if self.0 == other {
            return Some(0);
        }
        if self.1 == other {
            return Some(1);
        }
        if self.2 == other {
            return Some(2);
        }
        return None;
    }
    
    fn assign(&mut self, idx: usize, v: i32) {
        match idx {
            0 => self.0 = v,
            1 => self.1 = v,
            2 => self.2 = v,
            _ => assert!(idx < 3, "assign panic!"),
        }
    }
    
    fn anysame(&self) -> bool {
        self.0 == self.1 || self.0 == self.2 || self.1 == self.2
    }
    
    fn L1(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3)
    }
}


impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

fn alld2(beacons: &Vec<Point>) -> Vec<i64> {
    let mut d2ret: Vec<i64> = Vec::new();
    
    for i in 0..beacons.len() {
        for j in i+1..beacons.len() {
            d2ret.push((beacons[i] - beacons[j]).dist2());
        }
    }
    
    d2ret.sort();
    d2ret
}

fn count_overlap(di: &Vec<i64>, dj: &Vec<i64>) -> i32 {
    let mut nmatch = 0;
    let mut ii = 0;
    let mut jj = 0;
    while ii < di.len() && jj < dj.len() {
        if di[ii] == dj[jj] {
            nmatch += 1;
            ii += 1;
            jj += 1;
        }
        else if di[ii] < dj[jj] {
            ii += 1
        }
        else {
            jj += 1
        }
    }
    
    nmatch
}

fn search_matches(dists: &Vec<Vec<i64>>) -> Vec<Vec<usize>> {
    let ns = dists.len();
    let mut allmatch: Vec<Vec<usize>> = Vec::new();
    for i in 0..ns {
        let di = &dists[i];
        let npairmatches = dists.iter()
                            .enumerate()
                            .filter(|&(j,_)| j != i)
                            .map(|(j,dj)| (j,count_overlap(di, dj) ))
                            .collect::<Vec<(usize,i32)>>();
        
        let max = npairmatches.iter()
                              .map(|&(_,v)| v)
                              .max()
                              .unwrap();
        assert!(max == 12*11/2, "Warning, only {:?} matches", max);
        
        allmatch.push(
                npairmatches.iter()
                    .filter(|&(_,m)| *m == max)
                    .map(|(j,_)| *j)
                    .collect::<Vec<usize>>()
        );
    }
    allmatch
}

fn match_one_pair(b1: &Vec<Point>, b2: &Vec<Point>, nth: i32) -> Option<(usize,usize,usize,usize)> {
    let mut found = 0;
    for i in 0..b1.len(){
        if b1[i].anysame() {
            continue;
        }
        for ii in i+1..b1.len() {
            if b1[ii].anysame() {
                continue;
            }
            let di = (b1[i] - b1[ii]).dist2();
            
            for j in 0..b2.len() {
                if b2[j].anysame() {
                    continue;
                }
                for jj in j+1..b2.len() {
                    if b2[jj].anysame() {
                        continue;
                    }
                    let dj = (b2[j] - b2[jj]).dist2();
                    if di == dj {
                        found += 1;
                        if found > nth {
                            return Some((i,ii,j,jj));
                        }
                    }
                }
            }
        }
    }
    
    return None;
}

fn orientate(b1: &Vec<Point>, b2: &mut Vec<Point>) {
    let mut diff: Option<Point> = None;
    
    for nth in 0..=1000 {
        //println!("\n");
        let (i,ii,j,jj) = match_one_pair(&b1, &b2, nth).unwrap();
        //println!("{:?}", (i,ii,j,jj));

        let p1 = b1[i];
        let q1 = b1[ii];
        let p2 = b2[j];
        let q2 = b2[jj];

        // orientation
        let sep1 = p1 - q1;
        let sep2 = p2 - q2;

        //println!("{:?} {:?}", sep1, sep2);

        let (xperm,xflip) = match sep1.index(sep2.0) {
            Some(v) => (v,1),
            None => (sep1.index(-sep2.0).unwrap(), -1),
        };
        let (yperm,yflip) = match sep1.index(sep2.1) {
            Some(v) => (v,1),
            None => (sep1.index(-sep2.1).unwrap(), -1),
        };
        let (zperm,zflip) = match sep1.index(sep2.2) {
            Some(v) => (v,1),
            None => (sep1.index(-sep2.2).unwrap(), -1),
        };

        //println!("xperm, xflip {:?} {:?}", xperm, xflip);

        // apply
        //println!("{:?}", b2[0]);
        for bb2 in b2.iter_mut() {
            let old = bb2.clone();
            bb2.assign(xperm, old.0*xflip);
            bb2.assign(yperm, old.1*yflip);
            bb2.assign(zperm, old.2*zflip);
        }
        //println!("{:?}", b2[0]);

        // offset
        //println!("{:?} {:?} {:?} {:?}", b1[i], b1[ii], b2[j], b2[jj]);
        diff = Some(b2[j] - b1[i]);
        if b1[ii] != (b2[jj] - diff.unwrap()) {
            diff = Some(b2[jj] - b1[i]);
            assert!(false);
        }
        assert!(b1[ii] == (b2[jj] - diff.unwrap()));

        // now count how many are the same
        let nsame = b2.iter().filter(|p| b1.contains(&(**p - diff.unwrap()))).count();
        //println!("nsame = {}, nth = {}", nsame, nth);
        if nsame == 12 {
            break;
        }
        
        diff = None;
    }

    for bb2 in b2.iter_mut() {
        *bb2 -= diff.unwrap();
    }
    
    //println!("{:?} {:?} {:?} {:?}", b1[i], b1[ii], b2[j], b2[jj]);
}

fn orientate_all(beacons: &mut Vec<Vec<Point>>, matches: &Vec<Vec<usize>>) {
    let mut stack: Vec<(usize,usize)> = Vec::new();
    stack.extend( matches[0].iter().map(|&v| (0usize,v)) );
    
    let mut orientated = vec![false;beacons.len()];
    orientated[0] = true;
    
    println!("stack {:?}", stack);
    while let Some((i,j)) = stack.pop() {
        if !orientated[j] {
            orientate(&beacons[i].clone(), &mut beacons[j]);
            orientated[j] = true;
        }
        
        stack.extend(matches[j].iter()
                               .filter(|v| !orientated[**v])
                               .map(|&v| (j,v))
                               .rev()
        );
        //println!("stack {:?}", stack);
    }
    println!("orientated {:?} of {}",
        orientated.iter().map(|t| *t as i32).sum::<i32>(),
        orientated.len());
}

fn dedup(beacons: &mut Vec<Vec<Point>>) {
    for i in 0..beacons.len() {
        let sensor1 = beacons[i].clone();
        for j in i+1..beacons.len() {
            beacons[j].retain(|&bj| !sensor1.contains(&bj));
        }
    }
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();

    let mut beacons: Vec<Vec<Point>> = Vec::new();
    for line in txt.lines() {
        if line.starts_with("---") {
            beacons.push(Vec::new());
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        let coords = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        beacons.last_mut().unwrap().push(Point(coords[0],coords[1],coords[2],false));
    }
    
    let mut d2lists: Vec<Vec<i64>> = beacons.iter()
                                            .map(|s| alld2(&s))
                                            .collect();
    
    let matches = search_matches(&d2lists);
    println!("{:?}", matches);
    
    //for b in beacons.iter_mut() {
    //    b.push(Point(0,0,0,true));
    //}
    
    orientate_all(&mut beacons, &matches);
    
    //println!("{:?}", beacons[0]);
    //println!("{:?}", beacons[1]);
    
    let nbeacon: usize = beacons.iter().map(|b| b.len()).sum();
    println!("predup size: {}", nbeacon);
    
    let maxdist = beacons.iter().map(|v| v.last().unwrap())
                         .map(|p| beacons.iter()
                                         .map(|v2| v2.last().unwrap())
                                         .map(|q| (*p-*q).L1())
                                         .max()
                                         .unwrap()
                             ).max().unwrap();
    println!("maxdist {}", maxdist);
    
    dedup(&mut beacons);
    
    let nbeacon: usize = beacons.iter().map(|b| b.len()).sum();
    println!("final size: {}", nbeacon);
}
