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
    let mut h = vec![0; n];

    for i in 0..n {
        h[i] = read::<i64>();
    }

    let mut cnt = 0;
    for i in 0..n-1 {
        if h[i] > h[i+1] {
            if h[i] > h[i+1] + 1 {
                println!("No");
                return;
            } else {
                cnt += 1;
                if cnt > 1 {
                    println!("No");
                    return;
                }
            }
        } else {
            if h[i] < h[i+1] {
                cnt = 0;
            }
        }
    }

    println!("Yes");
}