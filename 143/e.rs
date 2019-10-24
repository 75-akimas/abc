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

use std::cmp::min;

fn main() {
    let n = read::<usize>();    
    let m = read::<usize>();    
    let l = read::<usize>();    
    let mut a = vec![0; m];
    let mut b = vec![0; m];
    let mut c = vec![0; m];
    let INF = 100_100_100_1;
    let mut re = vec![vec![INF; n]; n];
    for i in 0..m {
        a[i] = read::<usize>();
        b[i] = read::<usize>();
        c[i] = read::<usize>();
        re[a[i]-1][b[i]-1] = c[i];
        re[b[i]-1][a[i]-1] = c[i];
    }
    let q = read::<usize>();
    let mut s = vec![0; q];
    let mut t = vec![0; q];
    for i in 0..q {
        s[i] = read::<usize>();
        t[i] = read::<usize>();
    }
    println!("{:?} {:?} {:?}", a, b, c);
    println!("{:?} {:?}", s, t);
    println!("{:?}", re);
}