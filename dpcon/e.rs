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

    let mut w = vec![0; n];    
    let mut v = vec![0; n];    
    for i in 0..n {
        w[i] = read::<usize>();
        v[i] = read::<usize>();
    }

    let mut dp = vec![vec![100_100_100_1; 100_100 as usize]; n+1];

    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..100_100 as usize {
            if v[i] <= j {
                dp[i+1][j] = min(dp[i+1][j], dp[i][j-v[i]]+w[i]);
            }
            dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
        }
    }
    let mut ans :usize = 0;
    for i in 0..100_100 {
        if dp[n][i] <= k {
            ans = i;
        }
    }
    println!("{}", ans);
}
