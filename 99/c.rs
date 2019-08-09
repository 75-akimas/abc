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
    let n = read::<u32>();
    let mut a :Vec<u32> = Vec::new();
    a.push(1);
    let mut ii :u32 = 1;

    while ii * 6 <= n {
        ii *= 6;
        a.push(ii);
    }
    ii = 1;
    while ii * 9 <= n {
        ii *= 9;
        a.push(ii);
    }
    a.sort();

    let mut dp = vec![vec![1 << 29 as u32; (n+1) as usize]; (a.len()+1) as usize];
    for i in 0..a.len()+1 {
        dp[i][0] = 0;
    }
    for i in 0..a.len() as usize {
        for j in 0..(n+1) as usize {
            dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
            if j as u32 >= a[i] {
                dp[i+1][j] = min(dp[i+1][j], dp[i+1][j-a[i] as usize] + 1);
            }
        }
    }
    println!("{:?}", dp[a.len() as usize][n as usize]);
}