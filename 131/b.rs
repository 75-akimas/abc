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
    let l = read::<i64>();
    let mut ans = 0;
    let mut m:i64 = 10000;
    for i in 0..n {
        ans += l+i as i64;
        if m.abs() > (l+i as i64).abs() {
            m = l+i as i64;
        }
    }
    println!("{}", ans-m);
}

