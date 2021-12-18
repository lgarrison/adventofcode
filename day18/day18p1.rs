use std::fs;

fn mag(nums: &Vec<i32>, depths: &Vec<i32>) -> i32 {
    let mut sumstack: Vec<(i32,i32)> = Vec::new();
    
    sumstack.push((nums[0],depths[0]));
    for i in 1..nums.len() {
        //println!("{:?}", sumstack);
        sumstack.push((nums[i],depths[i]));
        
        while sumstack.len() > 1 &&
            sumstack[sumstack.len()-1].1 == sumstack[sumstack.len()-2].1 {
            let right = sumstack.pop().unwrap();
            let left = sumstack.pop().unwrap();

            sumstack.push( (3*left.0 + 2*right.0,
                            left.1 - 1)
                        );
        }
    }
    
    let sum = sumstack.pop().unwrap();
    assert!(sum.1 == 0);
    assert!(sumstack.is_empty());
    
    sum.0
}

fn reduce(nums: &mut Vec<i32>, depths: &mut Vec<i32>) {
    loop {
        let mut reduced = true;
        for i in 0..nums.len() {
            if depths[i] >= 5 {
                if i > 0 {
                    nums[i-1] += nums[i];
                }
                if i < nums.len()-2 {
                    nums[i+2] += nums[i+1];
                }
                nums[i] = 0;
                depths[i] -= 1;
                nums.remove(i+1);
                depths.remove(i+1);
                reduced = false;
                break;
            }
        }
        if !reduced {
            continue;
        }
        
        for i in 0..nums.len() {
            if nums[i] >= 10 {
                let old = nums[i];
                nums[i] = old/2;
                nums.insert(i+1,(old+1)/2);
                depths[i] += 1;
                depths.insert(i+1, depths[i]);
                reduced = false;
                break;
            }
        }
        
        if reduced {
            break;
        }
    }
}

fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    //let txt = "[[[[[9,8],1],2],3],4]";
    //let txt = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    /*let txt = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
";*/
    //let txt = "[[1,2],[[3,4],5]]";
    //let txt = "[9,1]";
    //let txt = "[[9,1],[1,9]]";
    //let txt = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
    
    let mut nums: Vec<Vec<i32>> = Vec::new();
    let mut depths: Vec<Vec<i32>> = Vec::new();
    
    for line in txt.lines() {
        let mut linenums = Vec::new();
        let mut linedepths = Vec::new();
        
        let mut i = 0usize;
        let mut depth = 0;
        
        while i < line.len() {
            match &line[i..i+1] {
                "[" => {
                    i += 1;
                    depth += 1;
                },
                "]" => {
                    i += 1;
                    depth -= 1;
                    assert!(depth >= 0);
                },
                "," => {
                    i += 1;
                }
                _ => { 
                    let numstr = *line[i..].split(|s| [',',']','['].contains(&s)).collect::<Vec<&str>>().first().unwrap();
                    i += numstr.len();
                    let num = numstr.parse::<i32>().unwrap();
                    linenums.push(num);
                    linedepths.push(depth);
                }
            }
        }
        assert!(depth == 0);
        
        nums.push(linenums);
        depths.push(linedepths);
    }
    
    //println!("{:?}", nums);
    //println!("{:?}", depths);
    
    let mut sum = nums.get_mut(0).unwrap().clone();
    let mut sumd = depths.get_mut(0).unwrap().clone();
    //reduce(&mut sum, &mut sumd);
    
    for i in 1..nums.len() {
        let num2 = nums.get_mut(i).unwrap();
        let d2 = depths.get_mut(i).unwrap();
        //reduce(&mut num2, &mut d2);
        
        for d in sumd.iter_mut() {
            *d += 1;
        }
        for d in d2.iter_mut() {
            *d += 1;
        }
        
        sum.append(num2);
        sumd.append(d2);
        //println!("\n{:?}", sum);
        //println!("{:?}", sumd);
        
        reduce(&mut sum, &mut sumd);
        //println!("\n{:?}", sum);
    }
    
    let summag = mag(&sum, &sumd);
    
    println!("{:?}", summag);
}
