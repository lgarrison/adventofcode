//use std::fs;
use std::collections::HashMap;

fn p1rolls(state: &(u8,u8)) -> Vec<(u8,u8)> {
    let mut res = Vec::new();
    
    for i1 in 1..=3 {
        for i2 in 1..=3 {
            for i3 in 1..=3 {
                let p1 = (state.0 - 1 + i1 + i2 + i3) % 10 + 1;
                res.push((p1,state.1));
            }
        }
    }
    
    res
}

fn p2rolls(state: &(u8,u8)) -> Vec<(u8,u8)> {
    let mut res = Vec::new();
    
    for i1 in 1..=3 {
        for i2 in 1..=3 {
            for i3 in 1..=3 {
                let p2 = (state.1 - 1 + i1 + i2 + i3) % 10 + 1;
                res.push((state.0,p2));
            }
        }
    }
    
    res
}


const WIN: u8 = 21;

type GameStates = HashMap<(u8,u8),HashMap<(u8,u8),u64>>;

fn step(states: &GameStates) -> GameStates {
    let mut res: GameStates = HashMap::new();
    
    for state in states.keys() {
        for oldscore in states[state].keys() {
            let ntrans = states[state][oldscore];
            
            // special case: states at score >= 21 do not transition!
            if oldscore.0 >= WIN || oldscore.1 >= WIN {
                let resscores = &mut res.entry(*state).or_insert(HashMap::new());
                *resscores.entry(*oldscore).or_insert(0u64) += ntrans;
            }
            else {
                for newstate1 in p1rolls(state){
                    for mut newstate in p2rolls(&newstate1) {
                        assert!(newstate.0 != state.0 && newstate.1 != state.1);

                        let mut newscore = *oldscore;
                        newscore.0 += newstate.0; 
                        if newscore.0 >= WIN {
                            newstate.1 = state.1;  // p2 won't move; technically doesn't matter
                        } else {
                            // will end if p1 wins first
                            newscore.1 += newstate.1;
                        }
                        assert!(newscore.0 != oldscore.0);

                        let newscores = &mut res.entry(newstate).or_insert(HashMap::new());

                        *newscores.entry(newscore).or_insert(0u64) += ntrans;

                        if newscore.0 >= WIN{
                            break;
                        }
                    }
                }
            }
        }
    }
    res
}

fn count_win(states: &GameStates) -> (u64,u64,u64) {
    let mut p1 = 0u64;
    let mut p2 = 0u64;
    let mut nstate = 0u64;
    for scoremap in states.values() {
        for score in scoremap.keys() {
            if score.0 >= WIN {
                assert!(score.1 < WIN);
                p1 += scoremap[score];
            } else if score.1 >= WIN {
                assert!(score.0 < WIN);
                p2 += scoremap[score];
            }
            else{
                panic!("bad score!");
            }
            nstate += scoremap[score];
        }
    }
    
    //if p1 > p2 { (p1,nstate) } else { (p2,nstate) }
    (p1,p2,nstate)
}

fn check_done(states: &GameStates) -> bool {
    for scoremap in states.values() {
        for score in scoremap.keys() {
            if score.0 < WIN && score.1 < WIN {
                return false;
            }
        }
    }
    true
}

fn main() {
    //let path = "input.txt";
    //let txt = fs::read_to_string(path).unwrap();
    
    let mut states: GameStates = HashMap::new();
    
    let mut initscore = HashMap::new();
    initscore.insert((0u8,0u8),1u64);
    states.insert((3u8,4u8),initscore);
    
    for state in states.keys() {
        println!("{:?}: {:?}", state, states[state]);
    }
    println!("\n");
    
    let mut niter = 0;
    loop {
        niter += 1;
        states = step(&states);
        if check_done(&states) {
            break;
        }
    }
    println!("did {:?} iterations", niter);
    
    for state in states.keys() {
        println!("{:?}: {:?}", state, states[state]);
    }
    
    let (n1, n2, nstate) = count_win(&states);
    println!("p1 has {:?} wins, p2 has {:?} wins", n1, n2);
    println!("reached {:?} states", nstate);
}
