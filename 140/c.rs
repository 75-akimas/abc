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
    let mut b = vec![1000000; n];
    let mut a = vec![0; n];

    for i in 0..n-1 {
        b[i] = read::<usize>();
    }

    let mut ans = b[0];
    for i in 1..n-1 {
        ans += min(b[i], b[i-1]);
    }
    println!("{}", ans + b[n-2])
}