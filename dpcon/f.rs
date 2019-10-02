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
    let s = read::<String>().chars().collect::<Vec<char>>();
    let t = read::<String>().chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i+1][j+1] = dp[i][j]+1;
            } else {
                dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
            }
        }
    }

    let mut i = s.len();
    let mut j = t.len();
    let mut v: Vec<char> = Vec::new();
    while i > 0 && j > 0 {
        if dp[i][j-1] == dp[i][j] {
            j-=1;
        } else if dp[i-1][j] == dp[i][j] {
            i-=1;
        } else if dp[i-1][j-1] < dp[i][j] {
            v.insert(0, s[i-1]);
            i-=1;
            j-=1;
        }
    }
    
    for i in 0..v.len() {
        print!("{}", v[i])
    }

    println!("");
}