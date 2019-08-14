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
    let n = read::<i32>();

    let mut ans = 0;

    for i in 1..n+1 {
        let mut j = i;
        let mut cnt = 0;
        while j > 0 {
            cnt += 1;
            j /= 10;
        }

        if cnt%2 == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}