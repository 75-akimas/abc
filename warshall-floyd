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

use std::cmp::min;

fn main() {
    let n = read::<usize>();    
    let m = read::<usize>();    
    let l = read::<usize>();    
    let INF = 100_100_100_1;
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..m {
        let a = read::<usize>()-1;
        let b = read::<usize>()-1;
        let c = read::<usize>();
        
        dist[b][a] = c;
        dist[a][b] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut dist2 = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            if dist[i][j] <= l {
                dist2[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist2[i][j] = std::cmp::min(dist2[i][j], dist2[i][k] + dist2[k][j]);
            }
        }
    }
    
    let q = read::<usize>();
    for i in 0..q {
        let s = read::<usize>()-1;
        let t = read::<usize>()-1;
        if dist2[s][t] == INF { println!("-1") } else { println!("{}", dist2[s][t]-1) }
    }
}