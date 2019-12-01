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
    let m1 = (read::<usize>(), read::<usize>());
    let m2 = (read::<usize>(), read::<usize>());
    if m1.0 == m2.0 {
        println!("0");
    } else {
        println!("1");
    }
}

