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
    let (l, r) = (read::<usize>(), read::<usize>());

    if r - l >= 2019 {
        println!("0");
        return;
    }
    let mut m = 2019;
    for i in l..r+1 {
        for j in i+1..r+1 {
            m = std::cmp::min(m, (i*j)%2019)
        }
    }
    println!("{}", m);
}