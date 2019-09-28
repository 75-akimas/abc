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

extern crate lazy_static;

use std::sync::RwLock;

lazy_static! {
    static ref to: Vec<usize>() = Vec::with_capacity(200005);
    static ref ans: Vec<usize>() = Vec::new();
}

fn dfs(v: usize, p: usize) {

} 

fn main() {

    let n = read::<usize>();
    let q = read::<usize>();

    let mut ab = vec![(0, 0); n];

    let to = Vec::with_capacity(200005);
    for i in 0..n-1 {
        ab[i] = (read::<usize>()-1, read::<usize>()-1);
        to[ab[i].0].push(ab[i].1);
        to[ab[i].1].push(ab[i].0);
    }

    let mut px = vec![(0, 0); q];
    let mut ans = vec![0; n];
    for i in 0..q {
        px[i] = (read::<usize>(), read::<usize>());
        ans[px[i].0-1] += px[i].1;
    }
    dfs(0, -1);
    for i in 0..n {
        print!("{} ", ans[i]);
    }
}