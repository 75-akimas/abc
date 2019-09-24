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

    let s = read::<String>().chars().collect::<Vec<char>>();
    let mut ans = 0;

    for i in 1..s.len() {
        if s[i] == s[i-1] {
            ans+=1;
        }       
    }

    println!("{}", min(ans + k*2, n-1))
}