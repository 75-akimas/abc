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

use std::cmp::{min, max};

fn main() {
    let a = read::<i32>();
    let b = read::<i32>();
    let c = read::<i32>();
    let op = a+b+c;
    let mut m = max(a, max(b, c));

    if op%2 != (3*m)%2 {
        m+=1;
    }

    println!("{}", (3*m - (op))/2);
}