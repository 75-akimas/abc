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
    let a = read::<usize>();
    let b = read::<usize>();

    let mut ans = 0;
    let mut cnt = 0;

    if b == 1 {
        println!("0");
        return;
    } else if b < a {
        println!("1");
        return;
    }

    while cnt + 1 < b {
        cnt += a-1;
        ans += 1;
    }

    println!("{}", ans);
}