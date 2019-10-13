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
    let a = (0..n).map(|_|  read::<usize>()).collect::<Vec<_>>();
    let sum = a.iter().fold(0, |sum, b| sum + b);

    let mut x1 = sum;
    for i in 0..n {
        if i % 2 == 1 {
            x1 -= 2*a[i];
        }
    }
    print!("{} ", x1);
    for i in 1..n {
        x1 = 2*a[i-1] - x1;
        print!("{} ", x1)
    }
}