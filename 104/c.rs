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

use std::cmp;

fn main() {
    let d = read::<usize>();
    let g = read::<i64>();

    let mut p = vec![0; d];
    let mut c = vec![0; d];

    for i in 0..d {
        p[i] = read::<i64>();
        c[i] = read::<i64>();
    }

    println!("{}", dfs(0, &mut p, c, 0, 0, g));
}

fn dfs(d: usize, mut p: Vec<i64>, c: Vec<i64>, g: i64, mut cnt :i64, G: i64) -> i64 {
    if d > p.len() {
        let mut res = 100000000000;
        for i in (0..p.len()).rev() {
            if c[i] > 0 {
                res = g + (*p[i]-1) * (i-1) as i64;
                if res < G {
                    res = 100000000000;
                }
                break;
            }
        }
        return res;
    }

    let using = *c[d] + *p[d] * (d+1) as i64;
    cnt += *p[d];
    p[d] = 0;

    return cmp::min(dfs(d+1, p, c, using, cnt, G)
                     , dfs(d+1, p, c, g, cnt, G));
}