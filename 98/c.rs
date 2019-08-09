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
    let s = read::<String>().chars().collect::<Vec<char>>();

    let mut c = vec![0; s.len()+1];
    let mut d = vec![0; s.len()+1];
    for i in 0..n {
        if s[i] == 'W' {
            c[i+1] = c[i] + 1;
        } else {
            c[i+1] = c[i];
        }
    }
    for i in 0..n {
        if s[s.len()-i-1] == 'E' {
            d[s.len()-i-1] = d[s.len()-i] + 1;
        } else {
            d[s.len()-i-1] = d[s.len()-i];
        }
    }

    let mut ans = 1 << 20;
    for i in 0..n {
        ans = min(ans, c[i+1] + d[i] - 1);
    }

    println!("{}", ans);
}