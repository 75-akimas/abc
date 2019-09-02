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
    let a = read::<String>().chars().collect::<Vec<char>>();
    let b = read::<String>().chars().collect::<Vec<char>>();
    let mut c = 0;
    for i in 0..3 {
        if a[i] == b[i] {
            c += 1;
        }
    }
    println!("{}", c)
}