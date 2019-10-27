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

fn main() {
    let a = read::<usize>();
    let b = read::<usize>();
    let n = read::<usize>();
    let aa:f64 = (2*n) as f64 / (a*b) as f64;
    println!("{}", ((b as f64/aa).atan())*180.0/std::f64::consts::PI)
}
