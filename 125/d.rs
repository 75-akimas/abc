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
    let mut a = (0..n).map(|_| read::<i64>()).collect::<Vec<i64>>();

    let mut cnt = 0;
    let mut m = 100_100_100_0;
    for i in 0..n {
        m = std::cmp::min(m, a[i].abs());
        if a[i] < 0 {
            cnt+=1;
        }
    }

    println!("{}", a.iter().fold(0, |sum, a| sum + a.abs()) - if cnt % 2 == 0 { 0 } else { 2*m });
}