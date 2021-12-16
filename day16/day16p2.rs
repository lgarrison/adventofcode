use std::fs;
use std::cmp;

fn low(n: u8) -> u8{
    if n == 8 { 0xFF } else { (1<<n)-1 }
}
fn _high(n: u8) -> u8{
    !low(8-n)
}

fn parse_literal(sig: &mut Vec<u8>, firstbit: &mut u8) -> (u64,usize) {
    let mut groups: Vec<u8> = Vec::new();
    loop {
        let group = parse_nbits(sig, firstbit, 5) as u8;
        groups.push(group & low(4));
        if group & (1<<4) == 0 {
            break;
        }
    }
    assert!(groups.len() <= 16);
    
    let mut res: u64 = 0;
    for (i,&group) in groups.iter().enumerate() {
        res += (group as u64) << ((groups.len() - 1 - i)*4);
    }
    
    (res, groups.len())
}

fn parse_nbits(sig: &mut Vec<u8>, firstbit: &mut u8, nbit: u8) -> u16 {
    assert!(nbit < 16);
    let mut b = 0;  // byte index
    let mut i = *firstbit;  // bit index
    let mut rem = nbit;  // remaining bits in group
    let mut group = 0u16;
    assert!(i < 8);
    loop {
        assert!(i < 8);
        let gobble = cmp::min(rem, 8-i);  // how many bits from this byte to consume
        assert!(gobble >= 1 && gobble <= 8);
        let lastbit = i + gobble - 1;  // last bit to use in this byte
        assert!(lastbit <= 7);
        let mut newbits: u8;
        newbits = sig[b];
        newbits >>= 7 - lastbit;
        newbits &= low(gobble);
        let newbits16 = (newbits as u16) << (rem - gobble);
        group |= newbits16;
        rem -= gobble;
        i += gobble;
        
        if lastbit == 7 {
            b += 1;
            i = 0;
        }
        
        if rem == 0 {
            // all done!
            break;
        }
    }
    
    // now increment the bit and byte offset
    *sig = sig[b..].to_vec();
    *firstbit = i;
    
    group
}

fn parse_packet(mut sig: &mut Vec<u8>, mut bitoff: &mut u8, res: &mut u64) -> u32 {
    let mut bitsparsed = 0u32;
    
    let ver = parse_nbits(&mut sig, &mut bitoff, 3);
    bitsparsed += 3;
    println!("\nverion = {:?}", ver);
    *res += ver as u64;
    
    let tid = parse_nbits(&mut sig, &mut bitoff, 3);
    bitsparsed += 3;
    println!("type = {:?}", tid);
    
    if tid == 4 {
        // literal
        let (lit,ngroup) = parse_literal(&mut sig, &mut bitoff);
        bitsparsed += (ngroup*5) as u32;
        println!("lit = {:?}", lit);
        *res = lit;
    } else {
        // operator
        let len_counts_pk = parse_nbits(&mut sig, &mut bitoff, 1) != 0;
        bitsparsed += 1;
        
        let lenbits = if len_counts_pk { 11u8 } else { 15u8 };
        let len = parse_nbits(&mut sig, &mut bitoff, lenbits);
        bitsparsed += lenbits as u32;
        println!("len = {:}, len_counts_pk = {}", len, len_counts_pk);
        
        let mut subres: Vec<u64> = Vec::new();
        if len_counts_pk {
            // number of packets
            for _ in 0..len {
                let mut sres: u64 = 0;
                bitsparsed += parse_packet(&mut sig, &mut bitoff, &mut sres);
                subres.push(sres);
            }
        } else {
            // number of bits in packets
            let mut subparsed = 0u32;
            loop {
                let mut sres: u64 = 0;
                subparsed += parse_packet(&mut sig, &mut bitoff, &mut sres);
                subres.push(sres);
                assert!(subparsed <= len as u32);
                if subparsed == len as u32 {
                    break;
                }
            }
            bitsparsed += subparsed;
        }
        
        *res = match tid {
                0 => { Some(subres.iter().sum()) },
                1 => { Some(subres.iter().product()) },
                2 => { Some(*subres.iter().min().unwrap()) },
                3 => { Some(*subres.iter().max().unwrap()) },
                5 => { Some((subres[0] > subres[1]) as u64) },
                6 => { Some((subres[0] < subres[1]) as u64) },
                7 => { Some((subres[0] == subres[1]) as u64) },
                _ => None,
            }.unwrap();
    }

    bitsparsed
}


fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    //let txt = "9C0141080250320F1802104A08";
    
    let mut sig: Vec<u8> = Vec::new();
    for i in (0..txt.len()).step_by(2) {
        sig.push(u8::from_str_radix(&txt[i..i+2], 16).unwrap());
    }
    
    let mut res = 0u64;
    parse_packet(&mut sig, &mut 0, &mut res);
    println!("\nres {}", res);
}
