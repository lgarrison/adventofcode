use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut ages: Vec<i64> = txt.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    let ndays = 256usize;
    let mut n_spawn_perday: Vec<i64> = vec![0;ndays];
    let newbornage = 8;
    let resetage = 6;
    let mut nfish = ages.len() as i64;
    
    for i in 0..ages.len() {
        for day in 0..ndays {
            ages[i] -= 1;
            if ages[i] < 0 {
                n_spawn_perday[day] += 1;
                ages[i] = resetage
            }
        }
    }
    
    for day in 0..ndays {
        nfish += n_spawn_perday[day];
        
        let mut thisage = newbornage;
        for d in (day+1)..ndays {
            thisage -= 1;
            if thisage < 0 {
                n_spawn_perday[d] += n_spawn_perday[day];
                thisage = resetage;
            }
        }
    }
    
    //println!("{:?}", n_spawn_perday);
    println!("{:?}", nfish);
}
