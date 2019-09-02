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

    let mut s = vec![vec!['a']; n];
    for i in 0..n {
        s[i] = read::<String>().chars().collect::<Vec<char>>();
        s[i].sort();
    }

    s.sort();

    let mut cnt:i64 = 0;
    let mut ans:i64 = 0;
    for i in 1..n {
        if s[i] == s[i-1] {
            cnt += 1;
        } else {
            ans += cnt * (cnt+1) / 2;
            cnt = 0;
        }
    }
    ans += cnt * (cnt+1) / 2;
    println!("{}", ans);
}