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
use std::cmp::max;
fn main() {
    let n = read::<usize>();
    let m = read::<usize>();

    let sum = n * m;

    if n == 1 && m == 1 {
        println!("{}", sum)
    } else if n == 1 || m == 1 {
        println!("{}", max(n, m) - 2)
    } else {
        println!("{}",sum - (2*(m+n) - 4));
    }
}