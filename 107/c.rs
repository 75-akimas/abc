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

    let mut x = vec![0; n];
    for i in 0..n {
        x[i] = read::<i64>();
    }

    let mut mini = std::i64::MAX;
    for i in 0..(n-k+1) {
        if x[i] < 0 && x[i+k-1] < 0 {
            mini = min(mini, x[i].abs());
        } else if x[i] >= 0 && x[i+k-1] >= 0 {
            mini = min(mini, x[i+k-1]);
        } else {
            if x[i].abs() > x[i+k-1] {
                mini = min(mini, 2*x[i+k-1] + x[i].abs());
            } else {
                mini = min(mini, 2*x[i].abs() + x[i+k-1]);
            }
        }

    }
    println!("{}", mini);
}