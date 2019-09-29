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

use std::cmp::max;

fn main() {
    let n = read::<usize>();
    let k = read::<i64>();

    let mut w = vec![0; n];    
    let mut v = vec![0; n];    
    for i in 0..n {
        w[i] = read::<i64>();
        v[i] = read::<i64>();
    }

    let mut dp = vec![vec![-1; (k*3) as usize]; n+1];
    for i in 0..n+1 {
        dp[i][0] = 0;
    }
    for i in 0..(k*3) as usize {
        dp[0][i] = 0;
    }
    for i in 0..n {
        for j in 1..(k+1) as usize {
            if w[i] <= j as i64 {
                dp[i+1][j] = max(dp[i][j], dp[i][j-w[i] as usize]+v[i]);
            } else {
                dp[i+1][j] = max(dp[i+1][j-1], dp[i][j]);
            }
        }
    }
    println!("{}", dp[n][k as usize]);
}
