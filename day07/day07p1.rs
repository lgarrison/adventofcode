use std::fs;

fn main () {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let hpos: Vec<i32> = txt.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
    let maxpos: i32 = *hpos.iter().max().unwrap();
    
    let minfuel: i32 = (0..=maxpos)
                        .map(|i| hpos
                            .iter()
                            .map(|p| (0..=(p - i).abs())
                                .sum::<i32>())
                            .sum()
                            )
                            //.collect::<Vec<i32>>()
                            .min()
                            .unwrap();
    println!("{:?}", minfuel);
}
