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
    let mut a = read::<String>().chars().collect::<Vec<char>>();

    let mut b = Vec::new();
    b.push(a[0]);
    for i in 1..n {
        if a[i] == a[i-1] {
            continue;
        }
        b.push(a[i]);
    }

    println!("{}", b.len());
}