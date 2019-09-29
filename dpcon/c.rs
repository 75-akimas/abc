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
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];
    for i in 0..n {
        a[i] = read::<i64>();
        b[i] = read::<i64>();
        c[i] = read::<i64>();
    }

    let mut dp = vec![vec![-1; n+2]; 3];
    dp[0][0] = a[0];
    dp[1][0] = b[0];
    dp[2][0] = c[0];

    for i in 1..n {
        let m = max(dp[1][i-1], dp[2][i-1]) + a[i];
        dp[0][i] = max(m, dp[0][i]);
        let m = max(dp[0][i-1], dp[2][i-1]) + b[i];
        dp[1][i] = max(m, dp[1][i]);
        let m = max(dp[1][i-1], dp[0][i-1]) + c[i];
        dp[2][i] = max(m, dp[2][i]);
    }

    println!("{}", max(dp[0][n-1], max(dp[1][n-1], dp[2][n-1])))
}