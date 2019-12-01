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
    let n = read::<usize>();
    let mut s = read::<String>().chars().collect::<Vec<char>>();
    let mut sh: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    let sl = s.len() as i64;
    for &i in s.iter() {
        *sh.entry(i).or_insert(0) += 1;
    }
    let mut sub = 0;
    for (&k, &v) in sh.iter() {
        if v > 1 {
            sub += binom_pascal(v as i64,1)
        }
        if v > 2 {
            sub += binom_pascal(v as i64,2)
        }
        if v > 3 {
            sub += binom_pascal(v as i64,3)
        }
    }

    println!("{:?}", sh);
    println!("{}", binom_pascal(sl, 3)-(sub as i64));
    println!("{}", binom_pascal(sl, 3));
}

fn binom_pascal( n: i64, k: i64 ) -> i64 {
    if k == 0 || k == n {
        return 1;
    }
    let k = k as usize;

    let mut p = vec![ 1 ];

    for _ in 0..n-1 {
        let mut c = vec![ 1 ];

        for x in p.windows(2) {
            c.push( x[0] + x[1] );
        }
        c.push( 1 );

        p = c;
    }

    p[k-1] + p[k]
}
