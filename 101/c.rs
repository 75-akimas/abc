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
    let k = read::<i64>();

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = read::<i64>();
    }
    let min = *a.iter().min().unwrap();
    let mut min_idx = vec![];

    for i in 0..n {
        if min != a[i] {
            min_idx.push(i);
        }
    }

    println!("{:?}", (min_idx.len() as i64 + (k-2)) / (k-1));
}