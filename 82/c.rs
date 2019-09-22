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

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let mut a = vec![0; n];
    let mut ans :i64 = 0;
    let mut h = HashMap::new();

    for i in 0..n {
        a[i] = read::<i64>();

        *h.entry(a[i]).or_insert(0) += 1;
    }

    for (k, v) in h {
        if k <= v {
            ans += v - k;
        } else {
            ans += v;
        }
    }
    println!("{}", ans);
}