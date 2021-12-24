//use std::fs;
use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const H: usize = 7;  // n_hallway
const HI: isize = H as isize;  // n_hallway
const N: usize = 23;  // total
const D: usize = 4;
const DI: isize = D as isize;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    slots: [usize;N],
    cost: usize,
    estcost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.estcost.cmp(&self.estcost)
            //.then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct Solver {
    dist_matrix: [usize;N*N],
}

impl Solver {
    fn fill_dist_matrix(&mut self) {
        for i in 0..N*N {
            self.dist_matrix[i] = usize::MAX;
        }

        for i in 0..N {
            let (iroom, _) = getroom(i as isize);
            // hall to room, room to room
            for j in H..N {
                let (jroom, _) = getroom(j as isize);
                if i >= H && iroom == jroom {
                    continue;
                }
                self.dist_matrix[i*N + j] = compute_dist(i,j);
                self.dist_matrix[j*N + i] = self.dist_matrix[i*N + j]
            }
        }
    }
    
    fn dist(&self, i: usize, j: usize, a: usize) -> usize {
        debug_assert!((1..=4).contains(&a));
        let mul = 10isize.pow((a-1) as u32);
        let d = self.dist_matrix[i*N + j];
        if d == usize::MAX {
            panic!("{} {} {}", i, j, a);
        }
        assert!(d != usize::MAX);
        
        d*mul as usize
    }
}

fn getroom(i: isize) -> (isize, isize) {
    let room: isize = (i - HI)/DI + 1;  // 1..=4
    let depth = ((i-HI) % DI) + 1;  // 1,2,3,4
    
    (room, depth)
}


fn est_cost(state: &State, solver: &Solver) -> usize {
    //return 0;
    
    // distance for each out-of-place
    let mut est = 0usize;
    
    // conservatively, we start placing at the front of the room
    // but if more than one needs to enter the room, then one of them
    // will have to move an extra 1 for sure
    let mut nroom = [0;4];
    
    // another opt: have to move out things in the way
    let mut bumped = [false;N];
    
    for i in 0..N {
        let a = state.slots[i];
        if a != 0 {
            let (room, depth) = getroom(i as isize);
            
            if i < H || a != room as usize {
                // starting from hall, or wrong room
                // move to room
                est += solver.dist(i, H + D*(a-1) + nroom[a-1], a);
                nroom[a-1] += 1;
            }
            
            if i >= H && a != room as usize {
                // if starting from wrong room, might have to move things that would not otherwise move
                let rstart = H + D*(room as usize - 1);
                for j in rstart..(rstart+depth as usize - 1) {
                    if !bumped[j] && state.slots[j] == room as usize {
                        let jdepth = j - rstart + 1;
                        let mul = 10isize.pow((state.slots[j]-1) as u32);
                        // jdepth + 1 up, then at least 2 back
                        let c: usize = ((jdepth + 1 + 2)*mul as usize) as usize;
                        //assert!(c == s);
                        est += c;
                        bumped[j] = true;
                    }
                }
            }
        }
    }
    for r in nroom.iter() {
        assert!(r <= &4);
    }
    est
}

fn compute_dist(iarg: usize, jarg: usize) -> usize {
    // only called on valid moves
    debug_assert!(iarg != jarg);
    
    let mut i = cmp::min(iarg,jarg) as isize;
    let j = cmp::max(iarg,jarg) as isize;
    
    debug_assert!(!((i < HI) && (j < HI)));  // no hall-to-hall
    
    let (iroom, idepth) = getroom(i);
    let (jroom, jdepth) = getroom(j);
    
    let mut d = 0;
    if i >= HI {
        debug_assert!((1..=4).contains(&iroom));
        debug_assert!((1..=4).contains(&jroom));
        debug_assert!(iroom != jroom);
        // room to room
        // always moving to right
        // do room to hall dist, then continue with hall to room
        d += idepth + 1;
        i = iroom + 1;
        debug_assert!(iroom != jroom);
    }
    
    // hall to room
    d += match i {
        //0..=HI => { ((jroom - i).abs()/2*2 + 1) + jdepth + if i == 0 || i == (HI-1) { 1 } else { 0 }},
        0..=1 => {
            (match jroom {
                1 => 1,
                2 => 3,
                3 => 5,
                4 => 7,
                _ => panic!("bad room {}",jroom),
            }) + jdepth + (2-i)/2
        },
        
        2..=2 => {
            (match jroom {
                1 => 1,
                2 => 1,
                3 => 3,
                4 => 5,
                _ => panic!("bad room {}",jroom),
            }) + jdepth
        },
        
        3..=3 => {
            (match jroom {
                1 => 3,
                2 => 1,
                3 => 1,
                4 => 3,
                _ => panic!("bad room {}",jroom),
            }) + jdepth
        },
        
        4..=4 => {
            (match jroom {
                1 => 5,
                2 => 3,
                3 => 1,
                4 => 1,
                _ => panic!("bad room {}",jroom),
            }) + jdepth
        },
        
        5..=6 => {
            (match jroom {
                1 => 7,
                2 => 5,
                3 => 3,
                4 => 1,
                _ => panic!("bad room {}",jroom),
            }) + jdepth + (i/2-2)
        },
        
        _ => { panic!("bad move from {} to {}", i, j) }
    };
    
    d as usize
}

impl State {
    fn check_room_to_hall(&self, room: usize, j: usize) -> bool {
        debug_assert!((0..=4).contains(&room));
        debug_assert!(j < H);
        let (jj,ii) = if j <= room { (j,room) } else { (room+1,j) };
        let mut clear = true;
        for k in jj..=ii {
            if self.slots[k] != 0 {
                clear = false;
                break;
            }
        }
        clear
    }
    
    fn check_hall_to_room(&self, j: usize, room: usize) -> bool {
        // just to the entrance
        debug_assert!((0..=4).contains(&room));
        debug_assert!(j < H);
        // don't include j because that's where we are!
        let (jj,ii) = if j <= room { (j+1,room) } else { (room+1,j-1) };
        let mut clear = true;
        for k in jj..=ii {
            if self.slots[k] != 0 {
                clear = false;
                break;
            }
        }
        clear
    }
    
    fn get_transitions(&self, solver: &Solver) -> Vec<State> {
        let mut trans = Vec::new();
        for (i,&a) in self.slots.iter().enumerate() {
            if a == 0 {
                continue;  // empty
            }
            if i >= H {
                // in a room
                
                let (room, depth) = getroom(i as isize);  // 1..=4
                debug_assert!((1..=DI).contains(&room));
                
                let rlast = H + D*(room as usize-1+1) - 1;
                let mut filled = true;
                for j in i..=rlast {
                    // self and all lower slots are in the right room
                    if self.slots[j] != room as usize {
                        filled = false;
                        break;
                    }
                }
                if filled {
                    //println!("{} filled", i);
                    continue;
                }
                
                let mut blocked = false;
                let rstart = H + D*(room as usize-1);
                for j in rstart..(rstart+depth as usize-1) {
                    // blocked by roommate
                    if self.slots[j] != 0 {
                        blocked = true;
                        break;
                    }
                }
                if blocked {
                    //println!("{} blocked, {} depth", i, depth);
                    continue;
                }
                
                // move from room to hall
                for j in 0..H {
                    if self.check_room_to_hall(room as usize,j) {
                        let mut tstate = self.clone();
                        tstate.slots[i] = 0;
                        debug_assert!(tstate.slots[j] == 0);
                        tstate.slots[j] = a;
                        tstate.cost += solver.dist(i,j,a);
                        trans.push(tstate)
                    }
                }
                
                // or room to the final room
                // unless interlopers are present
                let mut interlopers = false;
                for jdepth in 1..=D {
                    let j = H + D*(a as usize - 1) + (jdepth-1);
                    if self.slots[j] > 0 && self.slots[j] != a as usize {
                        interlopers = true;
                        break;
                    }
                }
                if interlopers {
                    continue;
                }
                
                for jdepth in 1..=D {
                    let jroom = a as isize;
                    let j = H + D*(jroom as usize - 1) + (jdepth-1);
                    
                    if self.slots[j] != 0 {
                        continue;  // occupied
                    }
                    
                    // move all the way in
                    if jdepth < D && self.slots[j+1] == 0 {
                        continue;
                    }
                    
                    if jroom == room {
                        continue;
                    }
                    
                    let (jj,ii) = if jroom < room { (jroom+1,room) } else { (room+1,jroom) };
                    let mut clear = true;
                    for k in jj..=ii {
                        if self.slots[k as usize] != 0 {
                            clear = false;
                            break;
                        }
                    }
                    
                    if clear {
                        let mut tstate = self.clone();
                        tstate.slots[i] = 0;
                        debug_assert!(tstate.slots[j] == 0);
                        tstate.slots[j] = a;
                        tstate.cost += solver.dist(i,j,a);
                        trans.push(tstate)
                    }
                }
            } else {
                // hallway, must move to own room
                let rstart = H + D*(a-1);
                let rend = H + D*(a+1-1) - 1;
                let mut to = None;
                for j in rstart..=rend {
                    if self.slots[j] == 0 {
                        // maybe, but keep looking for deeper loc or interloper
                        to = Some(j);  
                    } else if self.slots[j] != a {
                        // interloper in room, don't move
                        to = None;
                        break;
                    }
                }
                
                //if to.is_some() {
                //    println!("hall to room {:?}", to);
                //}
                
                match to {
                    Some(to) => if self.check_hall_to_room(i,a) {
                                    let mut tstate = self.clone();
                                    tstate.slots[i] = 0;
                                    debug_assert!(tstate.slots[to] == 0);
                                    tstate.slots[to] = a;
                                    tstate.cost += solver.dist(i,to,a);
                                    trans.push(tstate);
                                },
                    _ => ()
                }
            }
        }
        
        trans.sort_unstable_by_key(|t| t.cost);
        trans
    }
    
    fn win(&self) -> bool {
        for r in 1..=4 {
            for d in 1..=D {
                if self.slots[H + D*(r-1) + d-1] != r {
                    return false;
                }
            }
        }
        true
    }
}

impl Solver {
    fn solve(&self, state: &State, mut bestcost: usize, depth: usize) -> (usize,Vec<State>) {
        //println!("{:?}", state);
        let mut bestbranch = vec![state.clone()];
        if state.win() {
            //panic!("{:?}", state);
            return (state.cost, bestbranch);
        }
        
        let mut besttrans = None;
        for trans in state.get_transitions(&self) {
            let est = est_cost(&trans, &self);  // a lower bound
            if trans.cost + est >= bestcost {
                continue;
            }
            let (cost,branch) = self.solve(&trans, bestcost, depth + 1);
            assert!(cost >= trans.cost + est);
            if cost < bestcost {
                bestcost = cost;
                besttrans = Some(branch);
                //println!("new best {}", bestcost);
            }
        }
        if besttrans.is_some() {
            bestbranch.extend(besttrans.unwrap());
        }
        return (bestcost,bestbranch);
    }
    
    fn solve_queue(&self, state: &State) -> usize {
        let mut queue = BinaryHeap::new();
        // prime
        queue.push(state.clone());
        
        let mut bestcost = usize::MAX;
        while let Some(s) = queue.pop() {
            if s.win() {
                if s.cost < bestcost {
                    bestcost = s.cost;
                }
                continue;
            }
            for mut trans in s.get_transitions(&self) {
                let est = est_cost(&trans, &self);  // lower bound
                trans.estcost = trans.cost + est;
                if trans.estcost < bestcost {
                    queue.push(trans);
                }
            }
        }
        
        bestcost
    }
}

fn main() {
    // 39001 too low
    
    // example
    //let state = State{slots: [0,0,0,0,0,0,0, 2,4,4,1, 3,3,2,4, 2,2,1,3, 4,1,3,1], cost: 0, estcost: 0};
    // all true
    //let state = State{slots: [0,0,0,0,0,0,0, 1,1,1,1, 2,2,2,2, 3,3,3,3, 4,4,4,4], cost: 0, estcost: 0};
    // backed-up example
    //let state = State{slots: [1,1,0,0,0,1,4, 2,4,4,1, 0,2,2,2, 3,3,3,3, 0,0,0,4], cost: 0, estcost: 0};
    // input
    let state = State{slots: [0,0,0,0,0,0,0, 2,4,4,3, 2,3,2,1, 4,2,1,1, 4,1,3,3], cost: 0, estcost: 0};
    // bug check
    // 10016
    //let state = State{slots: [1,1,4,0,0,1,4, 0,0,0,1, 2,2,2,2, 3,3,3,3, 0,0,4,4], cost: 0, estcost: 0};
    // 25016
    //let state = State{slots: [1,1,0,0,0,1,4, 0,4,4,1, 2,2,2,2, 3,3,3,3, 0,0,0,4], cost: 0, estcost: 0};
    // 25056
    //let state = State{slots: [1,1,0,0,0,1,4, 2,4,4,1, 0,2,2,2, 3,3,3,3, 0,0,0,4], cost: 0, estcost: 0};
    // 34056
    // let state = State{slots: [1,1,4,0,0,1,4, 2,4,4,1, 0,2,2,2, 3,3,3,3, 0,0,0,0], cost: 0, estcost: 0};
    // 34061
    //let state = State{slots: [1,1,4,0,0,0,4, 2,4,4,1, 0,2,2,2, 3,3,3,3, 0,0,0,1], cost: 0, estcost: 0};
    // 34661
    //let state = State{slots: [1,1,4,0,0,0,4, 2,4,4,1, 0,2,2,2, 0,3,3,3, 0,0,3,1], cost: 0, estcost: 0};
    // 34791
    //let state = State{slots: [1,1,4,0,2,2,4, 2,4,4,1, 0,0,0,2, 0,3,3,3, 0,0,3,1], cost: 0, estcost: 0};
    // 34841
    //let state = State{slots: [1,1,4,2,2,2,4, 2,4,4,1, 0,0,0,0, 0,3,3,3, 0,0,3,1], cost: 0, estcost: 0};
    // 39841
    //let state = State{slots: [1,1,0,2,2,2,4, 2,4,4,1, 0,0,0,4, 0,3,3,3, 0,0,3,1], cost: 0, estcost: 0};
    // 39881
    //let state = State{slots: [1,1,0,0,2,2,4, 2,4,4,1, 0,0,2,4, 0,3,3,3, 0,0,3,1], cost: 0, estcost: 0};
    // 41081
    //let state = State{slots: [1,1,0,0,2,2,4, 2,4,4,1, 3,3,2,4, 0,0,0,3, 0,0,3,1], cost: 0, estcost: 0};
    // 41089
    //let state = State{slots: [1,0,0,0,2,2,4, 2,4,4,1, 3,3,2,4, 0,0,1,3, 0,0,3,1], cost: 0, estcost: 0};
    // 41119 *
    //let state = State{slots: [1,0,0,0,0,2,4, 2,4,4,1, 3,3,2,4, 0,2,1,3, 0,0,3,1], cost: 0, estcost: 0};
    // 41159
    //let state = State{slots: [1,0,0,0,0,0,4, 2,4,4,1, 3,3,2,4, 2,2,1,3, 0,0,3,1], cost: 0, estcost: 0};
    // 41169
    // let state = State{slots: [0,0,0,0,0,0,4, 2,4,4,1, 3,3,2,4, 2,2,1,3, 0,1,3,1], cost: 0, estcost: 0};
    // 44169
    //let state = State{slots: [0,0,0,0,0,0,0, 2,4,4,1, 3,3,2,4, 2,2,1,3, 4,1,3,1], cost: 0, estcost: 0};
    debug_assert!(state.slots.len() == N);
    
    let mut solver = Solver{dist_matrix: [0;N*N]};
    solver.fill_dist_matrix();
    
    let (bestcost, path) = solver.solve(&state, usize::MAX, 0);
    println!("best cost: {:?}", bestcost);
    println!("best path: {:?}", path);
    //let bestcost2 = solver.solve_queue(&state);
    //println!("best cost queue: {:?}", bestcost2);
}
