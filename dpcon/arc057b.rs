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

use std::cmp::{ min, max };

fn main() {
    let n = read::<usize>();
    let k = read::<usize>();

    let a = (0..n).map(|_| read::<usize>()).collect::<Vec<_>>();

    let mut dp = vec![100_100_100_10; n+1];
    dp[0] = 0;
    let mut sum = 0;
    for i in 0..n {
        for j in (0..n).rev() {
            if dp[j] == 100_100_100_10 {
                continue;
            }
            dp[j+1] = min(dp[j+1], dp[j] + if sum != 0 { dp[j]*a[i]/sum + 1} else { 1 });
        }
        sum += a[i];
    }
    let mut ans = 0;

    if sum == k {
        println!("1");
    } else {
        for i in 0..n {
            if dp[i+1] <= k {
                ans = max(ans, i+1);
            }
        }
        println!("{}", ans);
    }
}