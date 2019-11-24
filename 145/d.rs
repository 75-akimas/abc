use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let a = read::<i64>();
    let b = read::<i64>();
    let m_min = std::cmp::min(a, b);
    let m_max = std::cmp::max(a, b);

    let mut base = m_min - (m_max - m_min);

    if (m_min + m_max) % 3 != 0 {
        println!("0");
        return;
    }
    let hi = (m_max-base) / 2;
    base = base / 3;
    let top_n = base+hi;
    
    let mut top:i64 = 1;

    println!("{} {} {}",base+top_n, base, top_n);
    for i in 0..(base+top_n) as usize {
        top *= (i+1) as i64;
        top = top;
    }
    let mut under1:i64 = 1;
    for i in 0..top_n as usize {
        under1 *= (i+1) as i64;
        under1 = under1;
    }
    let mut under2:i64 = 1;
    for i in 0..base as usize {
        under2 *= (i+1) as i64;
        under2 = under2;
    }
    println!("{} {} {}", top, under1, under2);
    println!("{}", (top/(under1*under2)) % (100_000_000_7))
}
