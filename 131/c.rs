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

use std::cmp::{max, min};
fn main() {
    let a = read::<i64>();
    let b = read::<i64>();
    let c = read::<i64>();
    let d = read::<i64>();

    let x = c*d;
    let y = x / gcd(max(c, d), min(c, d));

    let bet = b-a+1;

    let dc = b / c;
    let dd = b / d;
    let dcd = b / y;

    let sdc = (a-1) / c;
    let sdd = (a-1) / d;
    let sdcd = (a-1) / y;

    println!("{}", bet - ((dc+dd-dcd) - (sdc+sdd-sdcd)));
}

fn gcd(a :i64, b :i64) -> i64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a%b);
}