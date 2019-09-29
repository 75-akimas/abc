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

    let mut key = vec![(0, 0); m];
    for i in 0..m {
        let a = read::<usize>();
        let b = read::<usize>();

        let mut s = 0;
        for j in 0..b {
            let c = read::<usize>() - 1;
            s |= 1<<c;
        }
        key[i] = (s, a);
    }

    let mut dp = vec![100_000_000_000; 1<<n];
    dp[0] = 0;
    for i in 0..1<<n {
        for j in 0..m {
            let t = i | key[j].0;
            let cost = dp[i] + key[j].1;
            dp[t] = min(dp[t], cost);
        }
    }
    let ans = dp[(1<<n) - 1];
    if ans == 100_000_000_000 {
        println!("-1")
    } else {
        println!("{}", dp[(1<<n) - 1]);
    }
}