//use std::fs;
use std::cmp;

const H: usize = 7;  // n_hallway
const HI: isize = H as isize;  // n_hallway
const N: usize = 15;  // total

#[derive(Debug, Clone)]
struct State {
    slots: [usize;N],
    cost: usize,
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
            // hall to room, room to room
            for j in H..N {
                // could remove same room to same room
                self.dist_matrix[i*N + j] = compute_dist(i,j);
                self.dist_matrix[j*N + i] = self.dist_matrix[i*N + j]
            }
        }
    }
    
    fn dist(&self, i: usize, j: usize, a: usize) -> usize {
        debug_assert!((1..=4).contains(&a));
        let mul = 10isize.pow((a-1) as u32);
        let d = self.dist_matrix[i*N + j];
        assert!(d != usize::MAX);
        
        d*mul as usize
    }
}

fn getroom(i: isize) -> (isize, isize) {
    let room: isize = (i + 1)/2 - (HI + 1)/2 + 1;  // 1..=4
    let depth = 2 - (i % 2);  // 1,2
    
    (room, depth)
}


fn est_cost(state: &State, solver: &Solver) -> usize {
    // distance for each out-of-place
    let mut est = 0usize;
    for i in 0..N {
        let a = state.slots[i];
        if a != 0 {
            let (room, depth) = getroom(i as isize);
            if a != room as usize {
                est += solver.dist(i, H + 2*(a-1), a);
            }
        }
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
                
                let room: isize = ((i as isize) + 1)/2 - (HI + 1)/2 + 1;  // 1..=4
                debug_assert!((1..=4).contains(&room));
                
                let ineigh = if i % 2 == 1 { i + 1 } else { i - 1 } as usize;
                if a == (room as usize) && self.slots[ineigh] == (room as usize) {
                    continue;  // room is solved!
                }
                
                if a == (room as usize) && i > ineigh {
                    continue; // all the way in, and in the right room
                }
                
                if self.slots[ineigh] != 0 && i > ineigh {
                    continue;  // blocked by roommate
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
                for jdepth in 1..=2 {
                    let jroom = a as isize;
                    let j = H + 2*(jroom as usize - 1) + (jdepth-1);
                    
                    if self.slots[j] != 0 {
                        continue;  // occupied
                    }
                    
                    // move all the way in
                    if jdepth == 1 && self.slots[j+1] == 0 {
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
                let r1 = 2*(a-1) + H;
                let r2 = r1 + 1;
                let to = if self.slots[r1] == 0 {
                             if self.slots[r2] == 0 {
                                 Some(r2)
                             }
                             else if self.slots[r2] == a {
                                 Some(r1)
                             } else {
                                 None
                             }
                         } else {
                             None
                         };
                
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
        
        //trans.sort_by(|a,b| b.cost.cmp(&a.cost));
        //trans.sort_by(|a,b| a.cost.cmp(&b.cost));
        trans.sort_unstable_by_key(|t| t.cost);
        trans
    }
    
    fn win(&self) -> bool {
           self.slots[H + 0] == 1
        && self.slots[H + 1] == 1
        && self.slots[H + 2] == 2
        && self.slots[H + 3] == 2
        && self.slots[H + 4] == 3
        && self.slots[H + 5] == 3
        && self.slots[H + 6] == 4
        && self.slots[H + 7] == 4
    }
}

impl Solver {
    fn solve(&self, state: &State, mut bestcost: usize) -> usize {
        //println!("{:?}", state);
        if state.win() {
            //panic!("{:?}", state);
            return state.cost;
        }
        for trans in state.get_transitions(&self) {
            let est = est_cost(&trans, &self);  // a lower bound
            if trans.cost + est >= bestcost {
                continue;
            }
            let cost = self.solve(&trans, bestcost);
            if cost < bestcost {
                bestcost = cost;
                //println!("new best {}", bestcost);
            }
        }
        return bestcost;
    }
}

fn main() {
    //let path = "input.txt";
    //let txt = fs::read_to_string(path).unwrap();
    
    //println!("{} {} {} {} {} {} {}", dist(0,7,1), dist(1,7,1), dist(2,7,1), dist(3,7,1), dist(4,7,1), dist(5,7,1), dist(6,7,1));
    
    //return;
    
    //let state = State{slots: [0,0,0,0,0,0,0, 2,1, 3,4, 2,3, 4,1], cost: 0};
    //let state = State{slots: [0,0,0,0,0,0,0, 1,1, 2,2, 3,3, 4,4], cost: 0};
    let state = State{slots: [0,0,0,0,0,0,0, 2,3, 2,1, 4,1, 4,3], cost: 0};
    debug_assert!(state.slots.len() == N);
    
    let mut solver = Solver{dist_matrix: [0;N*N]};
    solver.fill_dist_matrix();
    
    let bestcost = solver.solve(&state, usize::MAX);
    
    println!("best cost: {:?}", bestcost);
}
