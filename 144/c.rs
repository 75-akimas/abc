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

    let mut ans = 100_100_100_100_0;
    for i in 1..f64::sqrt(n as f64) as usize+2 {
        if n % i == 0 {
            ans = std::cmp::min(ans, i+n/i-2);
        }
    }
    println!("{}", ans);

}