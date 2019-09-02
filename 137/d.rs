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
    let m = read::<usize>();

    let mut ab = vec![(0usize, 0i64); n];
    let mut c = vec![vec![]; m as usize];

    for i in 0..n {
        ab[i].0 = read::<usize>();
        ab[i].1 = read::<i64>();

        if ab[i].0 <= m {
            c[ab[i].0 - 1].push(ab[i].1);
        }
    }

    let mut pq = std::collections::BinaryHeap::new();
    let mut ans = 0;
    for i in 0..c.len() {
        for &w in c[i].iter() {
            pq.push(w);
        }
        if let Some(w) = pq.pop() {
            ans += w;
        }
    }
    println!("{}", ans);
}