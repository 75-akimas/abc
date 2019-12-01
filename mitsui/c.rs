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
    let x = read::<usize>();

    let mut dp = vec![vec![0; 100001]; 7];
    dp[0][0] = 1;
    for i in 1..7 {
        for j in 0..100001 {
            if j >= 99+i && dp[i][j-i-99] == 1 {
                dp[i][j] = 1;
            }
            dp[i][j] = max(dp[i][j], dp[i-1][j]);
        }
    }
    println!("{}", dp[6][x]);
}
