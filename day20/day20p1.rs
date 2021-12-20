use std::fs;

fn read_image(lines: &Vec<&str>) -> (Vec<u16>,usize) {
    let pad = 1000;
    let iw = 100;
    let nw = 2*pad + iw;
    let mut image = vec![0u16;pad*nw];
    for line in lines {
        image.append(&mut vec![0u16;pad]);
        let mut img = line.chars()
                      .map(|c| if c == '.' { 0 } else { 1 })
                      .collect::<Vec<u16>>();
        assert!(img.len() == iw);
        image.append(&mut img);
        image.append(&mut vec![0u16;pad]);
    }
    image.append(&mut vec![0u16;pad*(pad + iw + pad)]);
    
    (image, nw)
}

fn apply(map: &Vec<u16>, img: &Vec<u16>, nw: usize) -> Vec<u16> {
    assert!(img.len() == nw*nw);
    let mut res = vec![0u16;img.len()];
    for i in 1..nw-1 {
        for j in 1..nw-1 {
            let x = i*nw + j;
            let m = (img[x-nw-1] << 8)
                    + (img[x-nw] << 7)
                    + (img[x-nw+1] << 6)
                    + (img[x-1] << 5)
                    + (img[x] << 4)
                    + (img[x+1] << 3)
                    + (img[x+nw-1] << 2)
                    + (img[x+nw] << 1)
                    + (img[x+nw+1] << 0);
            res[x] = map[m as usize];
        }
    }
    
    // borders
    let b = res[nw + 1];
    for i in 0..nw {
        res[i] = b;
        res[(nw-1)*nw + i] = b;
        res[i*nw] = b;
        res[i*nw + nw - 1] = b;
    }
    
    res
}

fn show(image: &Vec<u16>, nw: usize) {
    for i in image.chunks(nw) {
        println!("{}", i.iter().map(|v| if *v == 0 { '.' } else { '#' }).collect::<String>() );
    }
    println!("");
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut lines = txt.lines();
    let map: Vec<u16> = lines.next()
                            .unwrap()
                            .chars()
                            .map(|c| if c == '.' { 0 } else { 1 })
                            .collect();
    assert!(map.len() == 512);
    lines.next();
    
    let (image, nw) = read_image(&lines.collect::<Vec<&str>>());
    
    show(&image, nw);
    let mut new = apply(&map, &image, nw);
    show(&new, nw);
    for _ in 1..50 {
        new = apply(&map, &new, nw);
        //show(&new, nw);
    }
    
    show(&new, nw);
    
    //show(&image, nw);
    //show(&new, nw);
    
    println!("count: {}", new.iter().map(|v| *v as u64).sum::<u64>());
}
