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
    let nx = (read::<usize>(), read::<i64>());
    let mut x = vec![0; nx.0];
    x[0] = (read::<i64>() - nx.1).abs();
    let mut y = x[0];
    for i in 1..nx.0 {
        x[i] = (read::<i64>() - nx.1).abs();
        y = gcd(x[i], y);
    }
    println!("{}", y);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {a} else {gcd(b, a%b)}
}