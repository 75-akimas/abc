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
    let mut to: Vec<Vec<Edge>> = vec![Vec::new(); n];

    for i in 1..n {
        let (a, b) = (read::<usize>()-1, read::<usize>()-1);
        to[a].push(Edge{to: b, id: i-1});
        to[b].push(Edge{to: a, id: i-1});
    }
    let mut m = 0;
    for i in 1..n {
        m = std::cmp::max(m, to[i-1].len());
    }

    let mut ans: Vec<usize> = vec![0;n-1];
    dfs(0, n, n, &mut to, &mut ans);
    println!("{}", m);
    for i in 0..ans.len() {
        println!("{}", ans[i]+1);
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    id: usize
}

fn dfs(i: usize, p: usize, pc: usize, to: &mut Vec<Vec<Edge>>, ans: &mut Vec<usize>) {
    let mut col = 0;
    for u in 0..to[i].len() {
        if to[i][u].to == p {
            continue;
        }
        if pc == col {
            col+=1;
        }
        ans[to[i][u].id] = col;
        dfs(to[i][u].to.clone(), i, col, to, ans);
        col += 1;
    }
}
