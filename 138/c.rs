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
    let mut y:Vec<i64> = vec![0; n];

    for i in 0..n {
        y[i] = read::<i64>();
    }

    y.sort();

    let mut ans:f64 = y[0] as f64;
    for i in 1..n {
        ans = (ans + y[i] as f64) / 2.0;
    }

    println!("{}", ans)
}