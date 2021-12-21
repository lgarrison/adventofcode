use std::fs;

#[derive(Debug)]
struct D100 {
    lastroll: u32,
    nrolls: u32,
}

impl D100 {
    fn roll(&mut self) -> u32 {
        let roll = self.lastroll % 100 + 1;
        self.lastroll = roll;
        self.nrolls += 1;
        roll
    }
    
    fn roll3(&mut self) -> u32 {
        self.roll() + self.roll() + self.roll()
    }
}

fn main() {
    //let path = "input.txt";
    //let txt = fs::read_to_string(path).unwrap();
    
    let mut p1 = 3;
    let mut p2 = 4;
    
    let mut s1 = 0;
    let mut s2 = 0;
    
    let mut practdie = D100{lastroll: 100, nrolls: 0};
    loop {
        let roll3_1 = practdie.roll3();
        p1 = (p1 - 1 + roll3_1) % 10 + 1;
        s1 += p1;
        if s1 >= 1000 {
            break;
        }
        
        let roll3_2 = practdie.roll3();
        p2 = (p2 - 1 + roll3_2) % 10 + 1;
        s2 += p2;
        if s2 >= 1000 {
            break;
        }
    }
    
    println!("{:?} {:?} {:?} {:?} {:?}", s1, s2, p1, p2, practdie);
    println!("answer: {:?}", practdie.nrolls*s1);
}
