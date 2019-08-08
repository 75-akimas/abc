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
    let n = read::<usize>();
    let mut a = vec![0; n];
    let mut c = vec![0; n];
    for i in 0..n {
        a[i] = read::<i64>();
        c[i] += a[i] - (i+1) as i64;
    }
    c.sort();

    println!("{}", c.iter().map(|&s| (s - c[n/2]).abs()).sum::<i64>());
}