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
    let mut h = vec![0; n+3];
    for i in 0..n {
        h[i] = read::<i64>();
    }

    let mut d :Vec<i64> = vec![100_000_000_000_000; n+3];
    d[0] = 0;
    for i in 0..n {
        let w1 = h[i+1] - h[i];
        let w2 = h[i+2] - h[i];

        d[i+1] = min(d[i+1], d[i]+w1.abs());
        d[i+2] = min(d[i+2], d[i]+w2.abs());
    }

    println!("{}", d[n-1]);
}