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
    let a = read::<i64>();
    let b = read::<i64>();
    let c = read::<i64>();
    let mut x = read::<i64>();
    let mut y = read::<i64>();

    let m = min(x, y);
    let a1 = 2*c*max(x, y);
    let a2 = 2*c*m + (x-m)*a + (y-m)*b;
    let a3 = x*a + y*b;



    println!("{}", min(a1, min(a2, a3)));
}