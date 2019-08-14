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
    let mut a = vec![0; n+2];

    for i in 1..n+1 {
        a[i] = read::<i64>();
    }

    let mut sum = 0;
    for i in 1..n+2 {
        sum += (a[i-1] - a[i]).abs();
    }

    for i in 1..n+1 {
        if (a[i-1] <= a[i] && a[i] <= a[i+1]) || (a[i+1] <= a[i] && a[i] <= a[i-1]) {
            println!("{}", sum);
        } else {
            println!("{}", sum - min((a[i] - a[i-1]).abs(), (a[i+1] - a[i]).abs()) * 2);
        }
    }
}