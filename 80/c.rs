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
    let mut f = vec![vec![0; 10]; n];
    let mut p = vec![vec![0; 11]; n];

    for i in 0..n {
        for j in 0..10 {
            f[i][j] = read::<i64>();
        }
    }
    for i in 0..n {
        for j in 0..11 {
            p[i][j] = read::<i64>();
        }
    }

    let mut ans = -1_000_000_000;
    for o in 1..2i32.pow(10) {
        let mut c = vec![0; n];
        for i in 0..10 {
            if 1 << i & o != 0 {
                for j in 0..n {
                    if f[j][i] == 1 {
                        c[j] += 1;
                    }
                }
            }
        }
        let mut t_sum = 0;
        for i in 0..n {
            t_sum += p[i][c[i]];
        }

        if t_sum > ans {
            ans = t_sum;
        }
    }
    println!("{}", ans)
}