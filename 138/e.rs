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
    let (n, q) = (read::<usize>(), read::<usize>());

    let mut tree = vec![vec![]; n];
    for _i in 0..n-1 {
        let (a, b) = (read::<usize>(), read::<usize>());
        tree[a-1].push(b-1);
        tree[b-1].push(a-1);
    }

    let mut px = vec![0i64; n];
    for _i in 0..q {
        let (p, x) = (read::<usize>(), read::<i64>());
        px[p-1] += x
    }
    
    dfs(0, 0, &mut px, &tree);
    for i in 0..n {
        print!("{} ", px[i]);
    }
}

fn dfs(v: usize, par: usize, px: &mut Vec<i64>, tree: &Vec<Vec<usize>>) {
    for &w in tree[v].iter()  {
        if w == par {
            continue;
        }
        px[w] += px[v];
        dfs(w, v, px, tree);
    }
}