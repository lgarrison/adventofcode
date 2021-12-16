use std::fs;
use std::cmp;

fn low(n: u8) -> u8{
    if n == 8 { 0xFF } else { (1<<n)-1 }
}
fn high(n: u8) -> u8{
    !low(8-n)
}

fn parse_literal(sig: &mut Vec<u8>, firstbit: &mut u8) -> Vec<u8> {
    let mut b = 0;  // byte index
    let mut i = *firstbit;  // bit index
    let mut rem = 5;  // remaining bits in group
    let mut group = 0;
    let mut groups: Vec<u8> = Vec::new();
    loop {
        let gobble = cmp::min(rem, 8-i);  // how many bits from this byte to consume
        assert!(gobble >= 1 && gobble <= 8);
        //println!("gobble = {:}", gobble);
        let lastbit = i + gobble - 1;  // last bit to use in this byte
        assert!(lastbit <= 7);
        //println!("lastbit = {:}", lastbit);
        let mut newbits: u8;
        newbits = sig[b];
        newbits >>= 7 - lastbit;
        newbits &= low(gobble);
        newbits <<=  rem - gobble;
        group |= newbits;
        rem -= gobble;
        i += gobble;
        
        if lastbit == 7 {
            b += 1;
            i = 0;
        }
        
        if rem == 0 {
            groups.push(group & low(4));  // clear the flag bit
            println!("group = 0b{:05b}", group);
            if group & (1<<4) == 0 {
                break;  // last group!
            }
            group = 0;
            rem = 5;
        }
    }

    *sig = sig[b..].to_vec();
    *firstbit = i;
    
    groups
}

fn parse_nbits(sig: &mut Vec<u8>, firstbit: &mut u8, nbit: u8) -> u16 {
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

fn parse_packet(mut sig: &mut Vec<u8>, mut bitoff: &mut u8, versum: &mut u32) -> u32 {
    let mut bitsparsed = 0u32;
    
    let ver = parse_nbits(&mut sig, &mut bitoff, 3);
    bitsparsed += 3;
    println!("\nverion = {:?}", ver);
    *versum += ver as u32;
    
    let tid = parse_nbits(&mut sig, &mut bitoff, 3);
    bitsparsed += 3;
    println!("type = {:?}", tid);
    
    if tid == 4 {
        // literal
        let lit = parse_literal(&mut sig, &mut bitoff);
        bitsparsed += (lit.len()*5) as u32;
        println!("lit = {:?}", lit);
    } else {
        // operator
        let len_counts_pk = parse_nbits(&mut sig, &mut bitoff, 1) != 0;
        bitsparsed += 1;
        
        let lenbits = if len_counts_pk { 11u8 } else { 15u8 };
        let len = parse_nbits(&mut sig, &mut bitoff, lenbits);
        bitsparsed += lenbits as u32;
        println!("len = {:}, len_counts_pk = {}", len, len_counts_pk);
        
        if len_counts_pk {
            // number of packets
            for _ in 0..len {
                bitsparsed += parse_packet(&mut sig, &mut bitoff, versum);
            }
        } else {
            // number of bits in packets
            let mut subparsed = 0u32;
            loop {
                subparsed += parse_packet(&mut sig, &mut bitoff, versum);
                assert!(subparsed <= len as u32);
                if subparsed == len as u32 {
                    break;
                }
            }
            bitsparsed += subparsed;
        }
    }

    bitsparsed
}


fn main() {
    let path = "input.txt";
    let txt = fs::read_to_string(path).unwrap();
    
    let mut sig: Vec<u8> = Vec::new();
    for i in (0..txt.len()).step_by(2) {
        sig.push(u8::from_str_radix(&txt[i..i+2], 16).unwrap());
    }
    
    let mut versum = 0;
    parse_packet(&mut sig, &mut 0, &mut versum);
    println!("versum {}", versum);
}
