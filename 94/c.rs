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
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        x[i] = read::<i64>();
        y[i] = x[i];
    }
    x.sort();

    let m = x[(n as i32/2 - 1) as usize];
    let mm = x[(n as i32/2) as usize];
    for i in 0..n {
        if y[i] > m {
            println!("{}", m);
        } else {
            println!("{}", mm)
        }
    }
}