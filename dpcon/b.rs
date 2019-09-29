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
    let k = read::<usize>();

    let mut h = vec![0; n+k];
    for i in 0..n {
        h[i] = read::<i64>();
    }
    let mut dp = vec![100_000_000_000; n+k];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..k+1 {
            let w = h[i+j] - h[i];
            dp[i+j] = min(dp[i+j], dp[i] + w.abs());
        }
    }

    println!("{}", dp[n-1])
}